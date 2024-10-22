use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use crate::model::session::SessionDetail;

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub enum EventType {
    SessionStart,
    Settings,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SessionStartEvent {
    pub(crate) details: SessionDetail,
}