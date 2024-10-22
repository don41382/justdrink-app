use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use crate::model::license::{LicenseInfo};

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct Settings {
    pub(crate) app: AppDetails,
    pub(crate) user: SettingsUserDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct AppDetails {
    pub(crate) version: String,
    pub(crate) license_info: LicenseInfo
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SettingsUserDetails {
    pub(crate) next_break_duration_minutes: u32,
    pub(crate) active: bool,
    pub(crate) allow_tracking: bool,
    pub(crate) enable_on_startup: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsSystemDetails {
    pub(crate) last_update_check_date: DateTime<Utc>,
}
