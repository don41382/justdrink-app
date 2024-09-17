use crate::model::event::EventType::{SessionStart, Settings};
use crate::model::session::SessionDetail;
use crate::model::settings::SettingsDetails;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub enum EventType {
    SessionStart,
    Settings,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SessionStartEvent {
    pub(crate) details: SessionDetail,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SettingsEvent {
    pub(crate) details: SettingsDetails,
}