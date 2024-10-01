use crate::model::session::SessionDetail;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub enum EventType {
    SessionStart,
    Settings,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SettingsStartEvent {
    pub(crate) start_with_about: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SessionStartEvent {
    pub(crate) details: SessionDetail,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct AlertEvent {
    pub(crate) title: String,
    pub(crate) message: String,
}
