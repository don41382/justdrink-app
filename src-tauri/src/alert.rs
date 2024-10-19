use std::sync::TryLockResult;
use anyhow::Error;
use log::{error};
use serde_json::json;
use tauri::{AppHandle, Manager, Runtime, WindowEvent};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::webview::{PageLoadEvent, PageLoadPayload};
use tauri_plugin_aptabase::EventTracker;
use tauri_plugin_http::reqwest::blocking::ClientBuilder;
use tauri_specta::Event;
use urlencoding::encode;
use crate::SettingsDetailsState;
use crate::tracking::Tracking;

const WINDOW_LABEL: &str = "alert";

pub trait Alert {
    fn alert(&self, title: &str, message: &str, error: Option<anyhow::Error>, silence: bool) -> ();
}

impl Alert for AppHandle {
    fn alert(&self, title: &str, message: &str, error: Option<Error>, silence: bool) -> () {
        let allow_sending_error = self
            .state::<SettingsDetailsState>()
            .try_lock()
            .ok()
            .and_then(|settings| settings.as_ref().map(|s| s.allow_tracking))
            .unwrap_or(false);

        if let Some(e) = error {
            error!("error '{}' with message '{}', error: {:?}",title, message, e);
            if allow_sending_error {
                self.track_event(
                    title,
                    Some(json!({
                    "info": message.to_string(),
                    "error": e.to_string(),
                })),
                );
            }
        } else {
            error!("error '{}' with message '{}'", title, message);
            if allow_sending_error {
                self.track_event(title, Some(json!({
                "info": format!("{}", message)
            })));
            }
        }

        if !silence {
            match display_alert(self.app_handle(), title, message) {
                Ok(_) => {}
                Err(e) => {
                    error!("unable to display error {:?}", e);
                    self.exit(-1);
                }
            }
        }
    }
}

fn show_alert<R: Runtime>(app: &AppHandle<R>, title: String, message: String) -> Result<(), anyhow::Error> {
    let _window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App(format!("/alert?title={title}&message={message}", title = encode(&title), message = encode(&message)).into()),
    )
        .title("Oops, we didn't expect this")
        .visible(false)
        .transparent(true)
        .always_on_top(false)
        .decorations(false)
        .skip_taskbar(true)
        .resizable(true)
        .shadow(true)
        .visible(false)
        .build()?;

    Ok(())
}

#[specta::specta]
#[tauri::command]
pub fn close_error_window(window: tauri::Window) {
    window
        .close()
        .expect("alert window must exists and should never be closed");
}

fn display_alert<R>(app: &AppHandle<R>, title: &str, message: &str) -> Result<(), Error>
where
    R: Runtime,
{
    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Regular)?;

    show_alert(app, title.to_string(), message.to_string())?;

    Ok(())
}
