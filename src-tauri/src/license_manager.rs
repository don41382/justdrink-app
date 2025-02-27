use crate::alert::Alert;
use crate::app_config::AppConfig;
use crate::license_manager::response::PaymentStatus;
use crate::model::license::LicenseInfoStatus;
use crate::{license_manager, model, LicenseManagerState};
use chrono::Utc;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use tauri::http::StatusCode;
use tauri::{AppHandle, Manager, State, Window};
use tauri_plugin_http::reqwest::blocking::{Client, Response};

mod response {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

    #[allow(dead_code)]
    #[derive(Deserialize, Debug, Clone)]
    pub(crate) struct ErrorResponse {
        status: u16,
        pub error: String,
        pub message: String,
        path: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub(crate) enum PaymentStatus {
        #[serde(rename(deserialize = "START"))]
        Start,
        #[serde(rename(deserialize = "REQUIRE_INFO"))]
        RequireInfo,
        #[serde(rename(deserialize = "READY_TO_CAPTURE"))]
        ReadyToCapture,
        #[serde(rename(deserialize = "PAID"))]
        Paid,
        #[serde(rename(deserialize = "CANCELED"))]
        Canceled,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub(crate) struct PurchaseInfo {
        #[serde(rename(deserialize = "totalTrialDays"))]
        pub(crate) total_trial_days: u32,
        #[serde(rename(deserialize = "trialDaysLeft"))]
        pub(crate) trial_days_left: u32,
        #[serde(rename(deserialize = "purchasePrice"))]
        pub(crate) purchase_price: f64,
        #[serde(rename(deserialize = "paymentStatus"))]
        pub(crate) payment_status: PaymentStatus,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub(crate) enum LicenseStatus {
        #[serde(rename(deserialize = "ACTIVE_TRIAL"))]
        ActiveTrial,
        #[serde(rename(deserialize = "ACTIVE_PAID"))]
        ActivePaid,
        #[serde(rename(deserialize = "ACTIVE_APPLE_APP_STORE"))]
        ActiveAppleAppStore,
        #[serde(rename(deserialize = "EXPIRED"))]
        Expired,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug, Clone)]
    pub(super) struct Trial {
        #[serde(rename(deserialize = "startsAt"))]
        pub(super) starts_at: DateTime<Utc>,
        #[serde(rename(deserialize = "expiresAt"))]
        pub(super) expires_at: DateTime<Utc>,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub(super) struct Paid {
        #[serde(rename(deserialize = "licenseKey"))]
        pub(super) license_key: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub(super) struct Response {
        pub(super) status: LicenseStatus,
        pub(super) trial: Option<Trial>,
        pub(crate) paid: Option<Paid>,
        #[serde(rename(deserialize = "purchaseInfo"))]
        pub(crate) purchase_info: PurchaseInfo,
    }
}

#[derive(Debug, Clone)]
pub struct TrialDetails {
    pub expired_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PaidDetails {
    pub license_key: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ValidTypes {
    Trial(TrialDetails),
    Paid(PaidDetails),
    Full,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LicenseStatus {
    Valid(ValidTypes),
    Expired(String),
    Invalid(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LicenseData {
    pub status: LicenseStatus,
    pub purchase_info: response::PurchaseInfo,
}

impl LicenseStatus {
    pub fn is_active(&self) -> bool {
        match self {
            LicenseStatus::Valid(_) => true,
            LicenseStatus::Expired(_) => false,
            LicenseStatus::Invalid(_) => false,
        }
    }
}

impl PaymentStatus {
    fn to_model(&self) -> model::license::LicensePaymentStatus {
        match self {
            PaymentStatus::Start => model::license::LicensePaymentStatus::Start,
            PaymentStatus::RequireInfo => model::license::LicensePaymentStatus::RequireInfo,
            PaymentStatus::ReadyToCapture => model::license::LicensePaymentStatus::ReadyToCapture,
            PaymentStatus::Paid => model::license::LicensePaymentStatus::Paid,
            PaymentStatus::Canceled => model::license::LicensePaymentStatus::Canceled,
        }
    }
}

impl LicenseData {
    pub(crate) fn to_model(&self) -> model::license::LicenseData {
        let info = match &self.status {
            LicenseStatus::Valid(trial_types) => match trial_types {
                ValidTypes::Trial(details) => model::license::LicenseInfo {
                    status: model::license::LicenseInfoStatus::Trial,
                    license_key: None,
                    message: Some(format!(
                        "Your trial has {:?} days remaining",
                        crate::session_window::days_between(chrono::Utc::now(), details.expired_at)
                    )),
                },
                ValidTypes::Paid(details) => model::license::LicenseInfo {
                    status: LicenseInfoStatus::Paid,
                    license_key: Some(details.license_key.clone()),
                    message: Some("Your license is valid".to_string()),
                },
                ValidTypes::Full => model::license::LicenseInfo {
                    status: LicenseInfoStatus::Full,
                    license_key: None,
                    message: Some("Full-Version".to_string()),
                },
            },
            LicenseStatus::Expired(_) => model::license::LicenseInfo {
                status: LicenseInfoStatus::Invalid,
                license_key: None,
                message: Some("Your license has expired".to_string()),
            },
            LicenseStatus::Invalid(error) => model::license::LicenseInfo {
                status: LicenseInfoStatus::Invalid,
                license_key: None,
                message: Some(error.clone()),
            },
        };
        model::license::LicenseData {
            info,
            payment: model::license::LicensePaymentInfo {
                total_trail_days: self.purchase_info.total_trial_days,
                trial_days_left: self.purchase_info.trial_days_left,
                purchase_price: self.purchase_info.purchase_price,
                payment_status: self.purchase_info.payment_status.to_model(),
            },
        }
    }
}

pub struct LicenseManager {
    client: Client,
    pub device_id: model::device::DeviceId,
    status: Option<LicenseData>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum ServerRequestError {
    Other(anyhow::Error), // For other errors (e.g., non-200, non-400 responses)
}

impl LicenseManager {
    pub fn new(device_id: &model::device::DeviceId) -> Self {
        let client = Client::new();

        info!("LicenseManager started.");
        Self {
            client,
            device_id: device_id.clone(),
            status: None,
        }
    }

    fn validate(
        client: &Client,
        device_id: &model::device::DeviceId,
    ) -> Result<LicenseData, ServerRequestError> {
        info!("validating license");
        let url = format!(
            "{}/app/v1/license/validate?device-id={}",
            AppConfig::build().get_url(),
            device_id.get_hash_hex_id()
        );

        let response = client
            .post(&url)
            .header("origin", Self::origin())
            .body("")
            .send()
            .map_err(|err| {
                ServerRequestError::Other(anyhow::anyhow!(
                    "status: {:?}, source: {:?}, url: {:?}",
                    err.status(),
                    err.source(),
                    err.url()
                ))
            })?;

        Self::parse_response(&url, response)
    }

    fn origin() -> String {
        if cfg!(feature = "fullversion") {
            "APPLE_APP_STORE".to_string()
        } else {
            "FREEMIUM_APP".to_string()
        }
    }

    fn parse_response(url: &String, response: Response) -> Result<LicenseData, ServerRequestError> {
        match response.status() {
            StatusCode::OK => Self::parse_json(response).map_err(|err| {
                ServerRequestError::Other(anyhow::anyhow!(
                    "failed to parse response from url '{:?}': {:?}",
                    url,
                    err
                ))
            }),
            _ => Err(ServerRequestError::Other(anyhow::anyhow!(
                "failed license request, unknown error with: {:?}, url: {:?}",
                response.status(),
                response.url().path()
            ))),
        }
    }

    fn parse_json(response: Response) -> Result<LicenseData, ServerRequestError> {
        let response: response::Response = response.json().map_err(|err| {
            ServerRequestError::Other(anyhow::anyhow!("failed to parse response: {:?}", err))
        })?;

        let license_status = match response.status {
            response::LicenseStatus::ActiveTrial => {
                let trial = response.trial.ok_or_else(|| {
                    ServerRequestError::Other(anyhow::anyhow!(
                        "marked as active trial, but no details not found"
                    ))
                })?;
                LicenseStatus::Valid(ValidTypes::Trial(TrialDetails {
                    expired_at: trial.expires_at,
                }))
            }
            response::LicenseStatus::ActivePaid => {
                let paid = response.paid.ok_or_else(|| {
                    ServerRequestError::Other(anyhow::anyhow!(
                        "marked as active paid, but no details not found"
                    ))
                })?;
                LicenseStatus::Valid(ValidTypes::Paid(PaidDetails {
                    license_key: paid.license_key,
                }))
            }
            response::LicenseStatus::Expired => {
                LicenseStatus::Expired("You trial expired".to_string())
            }
            response::LicenseStatus::ActiveAppleAppStore => {
                if cfg!(feature = "fullversion") {
                    LicenseStatus::Valid(ValidTypes::Full)
                } else {
                    LicenseStatus::Invalid(
                        "This license only works with the Apple App Store Version.".to_string(),
                    )
                }
            }
        };
        Ok(LicenseData {
            status: license_status,
            purchase_info: response.purchase_info,
        })
    }

    pub fn get_status(
        &mut self,
        app_handle: &AppHandle,
        prevent_server_request: bool,
        force_request: bool,
    ) -> Result<LicenseData, String> {
        let do_request = self.status.is_none() || force_request;

        if do_request && !prevent_server_request {
            self.refresh_license_status(app_handle)?;
        }

        match &self.status {
            None => Err("No license - request prevented".to_string()),
            Some(license_dta) => Ok(license_dta.clone()),
        }
    }

    pub fn refresh_license_status(
        &mut self,
        app_handle: &AppHandle,
    ) -> Result<LicenseData, String> {
        match Self::validate(&self.client, &self.device_id) {
            Ok(license_data) => {
                self.status = Some(license_data.clone());
                Ok(license_data)
            }
            Err(err) => {
                warn!("License validation failed during get_status: {:?}", err);
                match err {
                    ServerRequestError::Other(e) => {
                        app_handle.alert(
                            "License Error",
                            "Unable to access the license server. Please try again later.",
                            Some(e),
                            true,
                        );
                        Err(
                            "Unable to access the license server. Please try again later."
                                .to_string(),
                        )
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub enum LicenseResultStatus {
    Success,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct LicenseResult {
    status: LicenseResultStatus,
    error: Option<String>,
}

#[specta::specta]
#[tauri::command]
pub fn request_license_status(
    app: AppHandle,
    license_manager: State<'_, LicenseManagerState>,
) -> Result<model::license::LicenseData, String> {
    license_manager
        .lock()
        .expect("a license manager for request license")
        .get_status(app.app_handle(), false, true)
        .map(|data| data.to_model())
}
