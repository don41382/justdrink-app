use crate::alert::Alert;
use crate::app_config::AppConfig;
use crate::model::license::LicenseInfo;
use crate::{model, LicenseManagerState};
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

impl LicenseStatus {
    pub fn is_active(&self) -> bool {
        match self {
            LicenseStatus::Valid(_) => true,
            LicenseStatus::Expired(_) => false,
            LicenseStatus::Invalid(_) => false,
        }
    }
}

pub struct LicenseManager {
    client: Client,
    pub device_id: model::device::DeviceId,
    status: Option<LicenseStatus>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum ServerRequestError {
    BadRequest(String),   // For BAD_REQUEST errors (business error)
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
    ) -> Result<LicenseStatus, ServerRequestError> {
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

    fn register_license(
        &mut self,
        license_key: String,
    ) -> Result<LicenseStatus, ServerRequestError> {
        info!("register license");
        let url = format!(
            "{}/app/v1/license/register?device-id={}&license-key={}",
            AppConfig::build().get_url(),
            self.device_id.get_hash_hex_id(),
            license_key
        );
        let response = self.client.post(&url).body("").send().map_err(|err| {
            ServerRequestError::Other(anyhow::anyhow!(
                "status: {:?}, source: {:?}, url: {:?}",
                err.status(),
                err.source(),
                err.url()
            ))
        })?;

        Self::parse_response(&url, response).and_then(|status| {
            match &status {
                LicenseStatus::Valid(_) => {
                    info!("register new license");
                    self.status = Some(status.clone());
                }
                invalid => {
                    info!("invalid response, not setting license: {:?}", invalid);
                }
            }
            Ok(status)
        })
    }

    fn reset_license(&mut self) -> Result<LicenseStatus, ServerRequestError> {
        info!("reset license");
        let url = format!(
            "{}/app/v1/license/reset?device-id={}",
            AppConfig::build().get_url(),
            self.device_id.get_hash_hex_id()
        );
        let response = self.client.post(&url).body("").send().map_err(|err| {
            ServerRequestError::Other(anyhow::anyhow!(
                "status: {:?}, source: {:?}, url: {:?}",
                err.status(),
                err.source(),
                err.url()
            ))
        })?;

        Self::parse_response(&url, response).and_then(|status| {
            self.status = Some(status.clone());
            Ok(status)
        })
    }

    fn parse_response(
        url: &String,
        response: Response,
    ) -> Result<LicenseStatus, ServerRequestError> {
        match response.status() {
            StatusCode::OK => Self::parse_json(response).map_err(|err| {
                ServerRequestError::Other(anyhow::anyhow!(
                    "failed to parse response from url '{:?}': {:?}",
                    url,
                    err
                ))
            }),
            StatusCode::BAD_REQUEST => {
                let error: response::ErrorResponse = response.json().map_err(|err| {
                    ServerRequestError::Other(anyhow::anyhow!(
                        "failed to parse error response from url '{:?}': {:?}",
                        url,
                        err
                    ))
                })?;
                warn!("failed to register license: {:?}", &error);
                Ok(LicenseStatus::Invalid(error.message))
            }
            _ => Err(ServerRequestError::Other(anyhow::anyhow!(
                "failed license request, unknown error with: {:?}, url: {:?}",
                response.status(),
                response.url().path()
            ))),
        }
    }

    fn parse_json(response: Response) -> Result<LicenseStatus, ServerRequestError> {
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
        Ok(license_status)
    }

    pub fn get_status(
        &mut self,
        app_handle: &AppHandle,
        prevent_server_request: bool,
        force_request: bool,
    ) -> LicenseStatus {
        let do_request = self.status.is_none() || force_request;

        if do_request && !prevent_server_request {
            self.refresh_license_status(app_handle);
        }

        match &self.status {
            None => LicenseStatus::Invalid("No license - request prevented".to_string()),
            Some(status) => status.clone(),
        }
    }

    pub fn refresh_license_status(&mut self, app_handle: &AppHandle) -> LicenseStatus {
        match Self::validate(&self.client, &self.device_id) {
            Ok(status) => {
                self.status = Some(status.clone());
                status
            }
            Err(err) => {
                warn!("License validation failed during get_status: {:?}", err);
                match err {
                    ServerRequestError::BadRequest(msg) => {
                        app_handle.alert(
                            "License Error",
                            &format!("Unable to validate the license: {}", msg),
                            None,
                            false,
                        );
                        LicenseStatus::Invalid(format!("Unable to validate the license: {}", msg))
                    }
                    ServerRequestError::Other(e) => {
                        app_handle.alert(
                            "License Error",
                            "Unable to access the license server. Please try again later.",
                            Some(e),
                            true,
                        );
                        LicenseStatus::Invalid(
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
pub fn settings_register_license(
    app_handle: AppHandle,
    license_key: String,
) -> model::license::LicenseInfo {
    license_run(app_handle, |lm| {
        lm.lock()
            .expect("license manager state")
            .register_license(license_key.clone())
    })
}

#[specta::specta]
#[tauri::command]
pub fn settings_reset_license(app_handle: AppHandle) -> LicenseInfo {
    license_run(app_handle, |lm| {
        lm.lock().expect("license manager state").reset_license()
    })
}

#[specta::specta]
#[tauri::command]
pub fn get_a_license(window: Window, app_handle: AppHandle) -> () {
    let url = format!(
        "{}/pricing?utm_source=app&utm_medium=settings",
        AppConfig::build().get_url()
    );
    match webbrowser::open(&url) {
        Ok(_) => {}
        Err(err) => {
            let error = format!(
                "I am sorry, we are not able to open up the browser for '{}'",
                url
            );
            app_handle.alert(
                "Could not open Browser",
                error.as_str(),
                Some(anyhow::anyhow!(err)),
                false,
            );
        }
    }
    window.close().expect("settings window to close");
}

fn license_run<F>(app_handle: AppHandle, run: F) -> model::license::LicenseInfo
where
    F: Fn(State<LicenseManagerState>) -> Result<LicenseStatus, ServerRequestError>,
{
    run(app_handle.state::<LicenseManagerState>())
        .unwrap_or_else(|err| {
            let error = match err {
                ServerRequestError::BadRequest(err) => {
                    anyhow::anyhow!(err.to_string())
                }
                ServerRequestError::Other(err) => err,
            };
            app_handle.alert(
                "License Error",
                "Unable to request license.",
                Some(error),
                true,
            );
            LicenseStatus::Invalid("Unable to access license server. Please try later.".to_string())
        })
        .to_license_info()
}
