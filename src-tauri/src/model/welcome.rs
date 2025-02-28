use crate::model::session::{DrinkCharacter, GenderType, SipSize};
use crate::model::settings::SettingsUserDetails;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct WelcomeUserSettings {
    pub(crate) next_break_duration_minutes: u32,
    pub(crate) drink_amount_ml: u32,
    pub(crate) sip_size: SipSize,
    pub(crate) character: DrinkCharacter,
    pub(crate) gender_type: GenderType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct WelcomeLoadSettings {
    pub(crate) user: Option<SettingsUserDetails>,
    pub(crate) device_id: String,
    pub(crate) backend_url: String,
}
