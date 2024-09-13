use crate::model::session::SessionDetail;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SessionStartEvent {
    pub(crate) details: SessionDetail,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SettingsEvent {
    pub(crate) name: String,
}