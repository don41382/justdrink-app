use crate::model::session::SessionDetail;
use crate::model::settings::Settings;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SessionStartEvent {
    pub(crate) details: SessionDetail,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SettingsEvent {
    pub(crate) settings: Settings,
}