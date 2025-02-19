use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub enum DrinkCharacter {
    YoungWoman,
    YoungMan,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub enum SipSize {
    BigSip,
    HalfCup,
    FullCup,
}


#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub enum GenderType {
    Male,
    Female,
    Other
}