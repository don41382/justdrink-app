use crate::model::session::SessionDetail;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SessionStart {
    pub(crate) details: SessionDetail,
}