use crate::license_manager::{LicenseStatus, ValidTypes};
use crate::model::device::DeviceId;
use crate::{license_manager, LicenseManagerState, SettingsDetailsState};
use log::{info, warn};
use serde_json::{json, Value};
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest::blocking::{Client, ClientBuilder};

pub(crate) struct Tracking {
    client: Client,
    app_handle: AppHandle,
    machine_id: DeviceId,
    app_version: String,
    platform: String,
    arch: String,
}

#[derive(Debug)]
pub enum Event {
    Install,
    DrinkReminder,
    SetTimer(u32),
}

impl Event {
    fn name(&self) -> String {
        match self {
            Event::Install => String::from("install"),
            Event::DrinkReminder => String::from("start_session"),
            Event::SetTimer(minutes) => String::from(format!("set_timer_{}", minutes)),
        }
    }
}

impl Tracking {
    pub fn new(device_id: &DeviceId, app_handle: &AppHandle) -> Result<Self, anyhow::Error> {
        let platform = tauri_plugin_os::platform().to_string();
        let arch = tauri_plugin_os::arch().to_string();
        Ok(Tracking {
            client: ClientBuilder::new().build()?,
            machine_id: device_id.clone(),
            app_version: app_handle
                .config()
                .clone()
                .version
                .unwrap_or("unknown".to_string()),
            app_handle: app_handle.clone(),
            platform,
            arch,
        })
    }

    pub fn send_tracking(&self, event: Event) {
        let state = self.app_handle.state::<LicenseManagerState>()
            .lock()
            .expect("send tracking requires license manager")
            .get_status(self.app_handle.app_handle(), true);
        let allow_tracking = {
            let settings = self.app_handle.state::<SettingsDetailsState>();
            let result = if let Ok(guard) = settings.try_lock() {
                guard.as_ref().map(|s| s.allow_tracking).unwrap_or(false)
            } else {
                false
            };
            result
        };
        if allow_tracking {
            info!("send event: {:?}", event);
            let event_data = json!([{
                "event": event.name(),
                "properties": {
                    "token": "9f58d004510c838794b55947a21a4658",
                    "app_version": self.app_version,
                    "platform": self.platform,
                    "arch": self.arch,
                    "distinct_id": self.machine_id.get_hash_hex_id(),
                    "license_state": state.to_license_status_name()
                }
            }]);
            let client_clone = self.client.clone();
            std::thread::spawn(move || {
                Self::send(&event_data, client_clone).unwrap_or_else(|e| {
                    warn!("error sending tracking event: {:?}", e);
                    ()
                })
            });
        }
    }

    fn send(event_data: &Value, client_clone: Client) -> Result<(), anyhow::Error> {
        let res = client_clone
            .post("https://api.mixpanel.com/track?ip=1&verbose=1")
            .header("Accept", "text/plain")
            .header("Content-Type", "application/json")
            .json(&event_data)
            .timeout(Duration::from_secs(10))
            .send()?;

        let json = res.error_for_status()?.json::<Value>()?;
        let status = json
            .get("status")
            .map(|v| v.as_u64().unwrap_or(0))
            .ok_or(anyhow::anyhow!("expected status from tracking request"))?;

        if status == 1 {
            Ok(())
        } else {
            Err(anyhow::anyhow!("error sending tracking event: {}", json))
        }
    }
}

trait LicenseConverter {
    fn to_license_status_name(&self) -> String;
}

impl LicenseConverter for license_manager::LicenseStatus {
    fn to_license_status_name(&self) -> String {
        match self {
            LicenseStatus::Valid(types) => match types {
                ValidTypes::Trial(_) => "trial".to_string(),
                ValidTypes::Paid(_) => "paid".to_string(),
                ValidTypes::Full => "full".to_string(),
            },
            LicenseStatus::Expired(_) => "expired".to_string(),
            LicenseStatus::Invalid(reason) => {
                format!("invalid_{}", reason)
            }
        }
    }
}
