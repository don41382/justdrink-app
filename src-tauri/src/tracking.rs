use std::mem::swap;
use std::sync::{Mutex};
use std::thread::spawn;
use std::time::Duration;
use log::{info, warn};
use serde_json::{json, Value};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest::blocking::{Client, ClientBuilder};
use crate::model::session::{SessionEndingReason};
use crate::model::settings::SettingsDetails;

pub(crate) struct Tracking {
    client: Client,
    app_handle: AppHandle,
    machine_id: String,
    platform: String,
    arch: String,
}

#[derive(Debug)]
pub enum Event {
    StartSession,
    SetTimer(u32),
    EndSession(SessionEndingReason),
}

impl Event {
    fn name(&self) -> String {
        match self {
            Event::StartSession => String::from("start_session"),
            Event::SetTimer(minutes) => String::from(format!("set_timer_{}", minutes)),
            Event::EndSession(end) => match end {
                SessionEndingReason::EndOfTime => String::from("session_end_time"),
                SessionEndingReason::UserEscape => String::from("session_end_user_escape")
            }
        }
    }
}

impl Tracking {
    pub fn new(app_handle: &AppHandle) -> Result<Self, anyhow::Error> {
        let id = machine_uid::get().unwrap_or_else(|e| {
            warn!("can't get machine uid: {}", e);
            "unknown".to_string()
        });
        let platform = tauri_plugin_os::platform().to_string();
        let arch = tauri_plugin_os::arch().to_string();
        Ok(Tracking {
            client: ClientBuilder::new().build()?,
            machine_id: id,
            app_handle: app_handle.clone(),
            platform,
            arch,
        })
    }

    pub fn send_tracking(&self, event: Event) {
        let allow_tracking = {
            let settings = self.app_handle.state::<Mutex<Option<SettingsDetails>>>();
            let result = if let Ok(guard) = settings.try_lock() {
                guard.as_ref().map(|s| s.allow_tracking).unwrap_or(false)
            } else {
                false
            };
            result
        };
        if allow_tracking {
            info!("send event: {:?}", event);
            let event_data = json!({
                "api_key": "phc_mkp1psX6jauuoZon4F0UhJFEeBmOFOrNUao4QftORej",
                "event": event.name(),
                "distinct_id": self.machine_id,
                "properties": {
                    "platform": self.platform,
                    "arch": self.arch
                }
            });
            let client_clone = self.client.clone();
            spawn(move || {
                Self::send(&event_data, client_clone).unwrap_or_else(|e| {
                    warn!("error sending tracking event: {:?}", e);
                    ()
                })
            });
        }
    }

    fn send(event_data: &Value, client_clone: Client) -> Result<(), anyhow::Error> {
        let res = client_clone.post("https://eu.i.posthog.com/capture/")
            .header("Content-Type", "application/json")
            .json(&event_data)
            .timeout(Duration::from_secs(10))
            .send()?;

        res.error_for_status()?;
        Ok(())
    }

}
