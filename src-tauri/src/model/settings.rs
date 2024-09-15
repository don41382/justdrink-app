use serde::{Deserialize, Serialize};
use specta::Type;
use std::time::Duration;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SettingsDetails {
    pub(crate) next_break_duration_minutes: u32,
    pub(crate) active: bool,
}
