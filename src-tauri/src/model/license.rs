use crate::license_manager::{LicenseStatus, ValidTypes};
use crate::model;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub enum LicenseInfoStatus {
    Trial,
    Paid,
    Full,
    Invalid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct LicenseInfo {
    pub(crate) status: LicenseInfoStatus,
    pub(crate) license_key: Option<String>,
    pub(crate) message: Option<String>,
}

impl LicenseStatus {
    pub(crate) fn to_license_info(&self) -> model::license::LicenseInfo {
        match self {
            LicenseStatus::Valid(trial_types) => match trial_types {
                ValidTypes::Trial(details) => LicenseInfo {
                    status: model::license::LicenseInfoStatus::Trial,
                    license_key: None,
                    message: Some(format!(
                        "Your trial has {:?} days remaining",
                        crate::session_window::days_between(chrono::Utc::now(), details.expired_at)
                    )),
                },
                ValidTypes::Paid(details) => LicenseInfo {
                    status: LicenseInfoStatus::Paid,
                    license_key: Some(details.license_key.clone()),
                    message: Some("Your license is valid".to_string()),
                },
                ValidTypes::Full => LicenseInfo {
                    status: LicenseInfoStatus::Full,
                    license_key: None,
                    message: Some("Full-Version".to_string()),
                },
            },
            LicenseStatus::Expired(_) => LicenseInfo {
                status: LicenseInfoStatus::Invalid,
                license_key: None,
                message: Some("Your license has expired".to_string()),
            },
            LicenseStatus::Invalid(error) => LicenseInfo {
                status: LicenseInfoStatus::Invalid,
                license_key: None,
                message: Some(error.clone()),
            },
        }
    }
}
