use crate::license_manager::{LicenseStatus, ValidTypes};
use crate::model::device::DeviceId;
use crate::{license_manager, LicenseManagerState, SettingsManagerState};
use log::{info, warn};
use serde_json::{json, Value};
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest::{Client, ClientBuilder};

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
    ResetSettings,
    DrinkReminder,
    WelcomeQuit(String),
}

impl Event {
    fn name(&self) -> String {
        match self {
            Event::Install => String::from("install"),
            Event::ResetSettings => String::from("reset_settings"),
            Event::DrinkReminder => String::from("start_session"),
            Event::WelcomeQuit(state) => String::from(format!("welcome_quit_{}", state)),
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

    pub async fn send_tracking(&self, event: Event) {
        let license_status = self
            .app_handle
            .state::<LicenseManagerState>()
            .get_status(self.app_handle.app_handle(), true, false)
            .await
            .map(|data | data.status)
            .unwrap_or_else(|err| LicenseStatus::Invalid(err));

        let allow_tracking = self
            .app_handle
            .state::<SettingsManagerState>()
            .get_settings()
            .map(|s| s.user.allow_tracking)
            .unwrap_or(false);
        if allow_tracking {
            info!("send event: {:?}", event);
            let event_data = json!([{
                "event": event.name(),
                "properties": {
                    "token": "21c5f5354133116affaafe40b4d316db",
                    "app_version": self.app_version,
                    "platform": self.platform,
                    "arch": self.arch,
                    "distinct_id": self.machine_id.get_hash_hex_id(),
                    "license_state": license_status.to_license_status_name()
                }
            }]);
            Self::send(&event_data, &self.client).await.unwrap_or_else(|e| {
                warn!("error sending tracking event: {:?}", e);
                ()
            })
        }
    }

    async fn send(event_data: &Value, client: &Client) -> Result<(), anyhow::Error> {
        let res = client
            .post("https://api.mixpanel.com/track?ip=1&verbose=1")
            .header("Accept", "text/plain")
            .header("Content-Type", "application/json")
            .json(&event_data)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        let json = res.error_for_status()?.json::<Value>().await?;
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

    pub fn device_id(&self) -> DeviceId {
        self.machine_id.clone()
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
