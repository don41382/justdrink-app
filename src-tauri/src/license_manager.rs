use chrono::{Utc};
use serde::Deserialize;
use tauri::AppHandle;
use tauri_plugin_http::reqwest::blocking::Client;
use crate::{model};
use crate::alert::Alert;
use crate::license_manager::LicenseStatus::Invalid;
use crate::model::device::DeviceId;

mod response {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

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

pub struct TrailDetails {
    pub expired_at: chrono::DateTime<Utc>,
}

pub struct PaidDetails {
    pub license_key: String,
}

pub enum ValidTypes {
    Trail(TrailDetails),
    Paid(PaidDetails),
}

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

impl LicenseManager {
    pub fn new(app_handle: &AppHandle, device_id: &model::device::DeviceId) -> Self {
        let client = Client::new();
        let status = match Self::validate(client.clone(), device_id) {
            Ok(license) => license,
            Err(err) => {
                app_handle.alert("License Error", "Unable to access the license server. Please check your internet connection.", Some(err), false);
                Invalid("Unable to access license server.".to_string())
            }
        };
        Self {
            client,
            device_id: device_id.clone(),
            status,
        }
    }

    pub fn refresh(&mut self) -> Result<(), anyhow::Error> {
        let status = Self::validate(self.client.clone(), &self.device_id)?;
        self.status = status;
        Ok(())
    }

    fn validate(client: Client, device_id: &model::device::DeviceId) -> Result<LicenseStatus, anyhow::Error> {
        let response =
            client
                .post(format!("https://motionminute.app/app/v1/license/validate?device-id={}", device_id.get_hash_hex_id()))
                .body("")
                .send()
                .map_err(|err| anyhow::anyhow!("failed to send license request: {:?}", err))?;

        let response = response.error_for_status()?;
        let response: response::Response = response.json().map_err(|err| anyhow::anyhow!("failed to parse license response: {:?}", err))?;

        let license_status = match response.status {
            response::LicenseStatus::ActiveTrial => {
                let trail = response.trail.ok_or_else(|| anyhow::anyhow!("marked as active trail, but no details not found"))?;
                LicenseStatus::Valid(ValidTypes::Trail(TrailDetails {
                    expired_at: trail.expires_at,
                }))
            }
            response::LicenseStatus::ActivePaid => {
                let paid = response.paid.ok_or_else(|| anyhow::anyhow!("marked as active paid, but no details not found"))?;
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
}
