use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SettingsDetails {
    pub(crate) next_break_duration_minutes: u32,
    pub(crate) active: bool,
    pub(crate) allow_tracking: bool,
    pub(crate) enable_on_startup: bool,
}
