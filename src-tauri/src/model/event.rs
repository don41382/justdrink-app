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

pub trait Event {
    fn event_type(&self) -> EventType;
    fn details(&self) -> &dyn std::any::Any;
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SessionStartEvent {
    pub(crate) details: SessionDetail,
}

impl Event for SessionStartEvent {
    fn event_type(&self) -> EventType {
        SessionStart
    }

    fn details(&self) -> &dyn std::any::Any {
        &self.details
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SettingsEvent {
    pub(crate) details: SettingsDetails,
}

impl Event for SettingsEvent {
    fn event_type(&self) -> EventType {
        Settings
    }

    fn details(&self) -> &dyn std::any::Any {
        &self.details
    }
}
