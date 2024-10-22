use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use crate::model::license::LicenseInfo;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub enum SessionEndingReason {
    EndOfTime,
    UserEscape,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SessionId(pub String);


#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct Exercise {
    pub(crate) id: SessionId,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) advices: Vec<String>,
    pub(crate) duration_s: u16,
    pub(crate) active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct SessionDetail {
    pub(crate) exercise: Exercise,
    pub(crate) license_info: LicenseInfo,
}
