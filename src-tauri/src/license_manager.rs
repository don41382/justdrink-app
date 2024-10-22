use anyhow::Error;
use chrono::{Utc};
use log::warn;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager, State};
use tauri::http::StatusCode;
use tauri_plugin_http::reqwest::blocking::{Client, Response};
use crate::{model, LicenseManagerState};
use crate::alert::Alert;
use crate::model::device::DeviceId;
use crate::model::license::{LicenseInfo, LicenseInfoStatus};

mod response {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

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
        #[serde(rename(deserialize = "EXPIRED"))]
        Expired,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub(super) struct Trail {
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
        pub(super) trail: Option<Trail>,
        pub(crate) paid: Option<Paid>,
    }
}

#[derive(Debug, Clone)]
pub struct TrailDetails {
    pub expired_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PaidDetails {
    pub license_key: String,
}

#[derive(Debug, Clone)]
pub enum ValidTypes {
    Trail(TrailDetails),
    Paid(PaidDetails),
}

#[derive(Debug, Clone)]
pub enum LicenseStatus {
    Valid(ValidTypes),
    Expired(String),
    Invalid(String),
}

pub struct LicenseManager {
    client: Client,
    device_id: DeviceId,
    status: LicenseStatus,
}

#[derive(Debug)]
enum ServerRequestError {
    BadRequest(String),  // For BAD_REQUEST errors (business error)
    Other(Error),        // For other errors (e.g., non-200, non-400 responses)
}

impl LicenseManager {
    pub fn new(app_handle: &AppHandle, device_id: &model::device::DeviceId) -> Self {
        let client = Client::new();
        let status = match Self::validate(client.clone(), device_id) {
            Ok(license) => license,
            Err(err) => {
                match err {
                    ServerRequestError::BadRequest(err) => {
                        app_handle.alert("License Error", format!("Unable to access the license server: {}", err).as_str(), None, false);
                    }
                    ServerRequestError::Other(err) => {
                        app_handle.alert("License Error", "Unable to access the license server.: Please try again later", Some(err), false);
                    }
                }
                LicenseStatus::Invalid("Unable to access license server.".to_string())
            }
        };
        Self {
            client,
            device_id: device_id.clone(),
            status,
        }
    }

    pub fn refresh(&mut self) -> Result<(), ServerRequestError> {
        let status = Self::validate(self.client.clone(), &self.device_id)?;
        self.status = status;
        Ok(())
    }

    fn validate(client: Client, device_id: &model::device::DeviceId) -> Result<LicenseStatus, ServerRequestError> {
        let url = format!("https://motionminute.app/app/v1/license/validate?device-id={}", device_id.get_hash_hex_id());
        let response =
            client
                .post(&url)
                .body("")
                .send()
                .map_err(|err| ServerRequestError::Other(anyhow::anyhow!("failed to send license request: {:?}", err)))?;

        Self::parse_response(&url, response)
    }

    fn register_license(&mut self, license_key: String) -> Result<LicenseStatus, ServerRequestError> {
        let url = format!("https://motionminute.app/app/v1/license/register?device-id={}&license-key={}", self.device_id.get_hash_hex_id(), license_key);
        let response =
            self.client
                .post(&url)
                .body("")
                .send()
                .map_err(|err| ServerRequestError::Other(anyhow::anyhow!("failed to send license request: {:?}", err)))?;

        Self::parse_response(&url, response)
            .and_then(|status| {
                self.status = status.clone();
                Ok(status)
            })
    }

    fn reset_license(&mut self) -> Result<LicenseStatus, ServerRequestError> {
        let url = format!("https://motionminute.app/app/v1/license/reset?device-id={}", self.device_id.get_hash_hex_id());
        let response =
            self.client
                .post(&url)
                .body("")
                .send()
                .map_err(|err| ServerRequestError::Other(anyhow::anyhow!("failed to send license request: {:?}", err)))?;

        Self::parse_response(&url, response)
            .and_then(|status| {
                self.status = status.clone();
                Ok(status)
            })
    }

    fn parse_response(url: &String, response: Response) -> Result<LicenseStatus, ServerRequestError> {
        match response.status() {
            StatusCode::OK => {
                Self::parse_json(response)
                    .map_err(|err|
                        ServerRequestError::Other(anyhow::anyhow!("failed to parse response from url '{:?}': {:?}", url, err))
                    )
            }
            StatusCode::BAD_REQUEST => {
                let error: response::ErrorResponse =
                    response.json()
                        .map_err(|err|
                            ServerRequestError::Other(anyhow::anyhow!("failed to parse error response from url '{:?}': {:?}", url, err))
                        )?;
                warn!("failed to register license: {:?}", &error);
                Ok(LicenseStatus::Invalid(error.message))
            }
            _ => {
                Err(ServerRequestError::Other(anyhow::anyhow!("failed license request, unknown error with: {:?}, url: {:?}", response.status(), response.url())))
            }
        }
    }

    fn parse_json(response: Response) -> Result<LicenseStatus, ServerRequestError> {
        let response: response::Response =
            response
                .json()
                .map_err(|err| ServerRequestError::Other(anyhow::anyhow!("failed to parse response: {:?}", err)))?;

        let license_status = match response.status {
            response::LicenseStatus::ActiveTrial => {
                let trail = response.trail.ok_or_else(|| ServerRequestError::Other(anyhow::anyhow!("marked as active trail, but no details not found")))?;
                LicenseStatus::Valid(ValidTypes::Trail(TrailDetails {
                    expired_at: trail.expires_at,
                }))
            }
            response::LicenseStatus::ActivePaid => {
                let paid = response.paid.ok_or_else(|| ServerRequestError::Other(anyhow::anyhow!("marked as active paid, but no details not found")))?;
                LicenseStatus::Valid(ValidTypes::Paid(PaidDetails {
                    license_key: paid.license_key,
                }))
            }
            response::LicenseStatus::Expired => {
                LicenseStatus::Expired("You trail expired".to_string())
            }
        };
        Ok(license_status)
    }

    pub fn get_status(&self) -> LicenseStatus {
        self.status.clone()
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
pub fn settings_register_license(app_handle: AppHandle, license_key: String) -> model::license::LicenseInfo {
    license_run(app_handle, |lm|
        lm
            .lock()
            .expect("license manager state")
            .register_license(license_key.clone()),
    )
}

#[specta::specta]
#[tauri::command]
pub fn settings_reset_license(app_handle: AppHandle) -> LicenseInfo {
    license_run(app_handle, |lm|
        lm
            .lock()
            .expect("license manager state")
            .reset_license(),
    )
}


pub fn license_run<F>(app_handle: AppHandle, run: F) -> model::license::LicenseInfo
where
    F: Fn(State<LicenseManagerState>) -> Result<LicenseStatus, ServerRequestError>,
{
    run(app_handle.state::<LicenseManagerState>())
        .unwrap_or_else(|err| {
            let error = match err {
                ServerRequestError::BadRequest(err) => {
                    anyhow::anyhow!(err.to_string())
                }
                ServerRequestError::Other(err) => {
                    err
                }
            };
            app_handle.alert("License Error", "Unable to request license.", Some(error), true);
            LicenseStatus::Invalid("Unable to access license server. Please try later.".to_string())
        }).to_license_info()
}


pub fn to_license_result(license_status: LicenseStatus) -> LicenseResult {
    match license_status {
        LicenseStatus::Valid(ValidTypes::Paid(_details)) => {
            LicenseResult {
                status: LicenseResultStatus::Success,
                error: None,
            }
        }
        LicenseStatus::Expired(msg) => {
            LicenseResult {
                status: LicenseResultStatus::Error,
                error: Some(msg),
            }
        }
        LicenseStatus::Invalid(msg) => {
            LicenseResult {
                status: LicenseResultStatus::Error,
                error: Some(msg),
            }
        }

        LicenseStatus::Valid(ValidTypes::Trail(_)) => {
            LicenseResult {
                status: LicenseResultStatus::Success,
                error: None,
            }
        }
    }
}
