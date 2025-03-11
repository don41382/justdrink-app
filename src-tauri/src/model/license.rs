use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub enum LicenseInfoStatus {
    Trial,
    Paid,
    Full,
    Invalid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub(crate) enum LicensePaymentStatus {
    GoToCheckout,
    ReadyToCapture,
    Paid,
    Canceled,
    Error
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub(crate) struct LicensePaymentInfo {
    pub(crate) total_trail_days: u32,
    pub(crate) trial_days_left: u32,
    pub(crate) purchase_price: f64,
    pub(crate) payment_status: LicensePaymentStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct LicenseInfo {
    pub(crate) status: LicenseInfoStatus,
    pub(crate) license_key: Option<String>,
    pub(crate) message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct LicenseData {
    pub(crate) payment: LicensePaymentInfo,
    pub(crate) info: LicenseInfo
}
