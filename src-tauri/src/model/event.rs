use serde::{Deserialize, Serialize};
use specta::Type;
use crate::model::session::{DrinkCharacter, SipSize};

#[derive(Serialize, Deserialize, Debug, Clone, Type, tauri_specta::Event)]
pub struct SessionStartEvent {
    pub(crate) selected_drink_character: DrinkCharacter,
    pub(crate) sip_size: SipSize,
}