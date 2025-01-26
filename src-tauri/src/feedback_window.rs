use log::{error};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager, Runtime, State};
use tauri_plugin_http::reqwest::blocking::Client;
use crate::{model, FeedbackSenderState, SettingsSystemState};
use crate::app_config::AppConfig;
use crate::model::device::DeviceId;
use crate::settings_system::SettingsSystem;

pub(crate) const WINDOW_LABEL: &'static str = "feedback";

pub fn show<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.show()?;
        window.set_focus()?;
    } else {
        let _window = tauri::WebviewWindowBuilder::new(
            app,
            WINDOW_LABEL,
            tauri::WebviewUrl::App("/feedback".into()),
        )
            .title("Motion Minute - Actionbar")
            .center()
            .transparent(true)
            .decorations(false)
            .shadow(true)
            .resizable(false)
            .inner_size(1024.0, 768.0)
            .visible(false)
            .build()?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedbackRequest {
    device_id: String,
    feedback: String,
    rating: FeedbackRate,
}

pub(crate) struct FeedbackSender {
    device_id: DeviceId,
    client: Client,
}

impl FeedbackSender {
    pub fn new(device_id: &model::device::DeviceId) -> Self {
        FeedbackSender {
            client: Client::new(),
            device_id: device_id.clone(),
        }
    }

    pub fn send_feedback(&self, feedback: String, rating: FeedbackRate) -> Result<(), anyhow::Error> {
        let feedback_request = FeedbackRequest {
            device_id: self.device_id.get_hash_hex_id(),
            feedback,
            rating,
        };

        let response = self.client
            .post(format!("{}/app/v1/feedback", AppConfig::build().get_url()))
            .json(&feedback_request)
            .send();

        response?.error_for_status()?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub enum FeedbackRate {
    UNKNOWN,
    BAD,
    OK,
    AWESOME,
}

#[specta::specta]
#[tauri::command]
pub fn feedback_window_send_feedback(app: AppHandle, feedback: String, rating: FeedbackRate, feedback_sender: State<FeedbackSenderState>, settings_system: State<SettingsSystemState>) -> Result<(), String> {
    feedback_sender.send_feedback(feedback, rating).map_err(|err| {
        let msg = format!("unable to send feedback: {}", err);
        error!("{}", msg);
        msg
    })?;
    settings_system.lock().as_mut().expect("settings_system lock required").feedback_given(&app);
    Ok(())
}

pub trait FeedbackDisplay {
    fn should_show_feedback(&self) -> bool;
}

impl FeedbackDisplay for SettingsSystem {
    fn should_show_feedback(&self) -> bool {
        let ask_intervals = [3u32, 13u32, 21u32, 34u32, 55u32, 89u32, 100u32, 144u32, 233u32];
        !self.settings().feedback_provided && ask_intervals.contains(&self.settings().session_count)
    }
}