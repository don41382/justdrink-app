use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SessionDetail {
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) duration_s: u16,
}