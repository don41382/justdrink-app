use anyhow::Error;
use log::error;
use serde_json::json;
use tauri::{AppHandle, Manager, Runtime};

use crate::SettingsManagerState;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri_plugin_aptabase::EventTracker;
use urlencoding::encode;

const WINDOW_LABEL: &str = "alert";

pub trait Alert {
    fn alert(&self, title: &str, message: &str, error: Option<anyhow::Error>, silence: bool) -> ();
}

impl Alert for AppHandle {
    fn alert(&self, title: &str, message: &str, error: Option<anyhow::Error>, silence: bool) -> () {
        let allow_sending_error = self
            .state::<SettingsManagerState>()
            .get_settings()
            .map(|s| s.user.allow_tracking)
            .unwrap_or(false);

        if let Some(e) = error {
            error!(
                "error '{}' with message '{}', error: {:?}",
                title, message, e
            );
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
                self.track_event(
                    title,
                    Some(json!({
                        "info": format!("{}", message)
                    })),
                );
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

fn show_alert<R: Runtime>(
    app: &AppHandle<R>,
    title: String,
    message: String,
) -> Result<(), anyhow::Error> {
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.close()?;
        return Ok(());
    }

    let _window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App(
            format!(
                "/alert?title={title}&message={message}",
                title = encode(&title),
                message = encode(&message)
            )
            .into(),
        ),
    )
    .title("Oops, we didn't expect this")
    .visible(false)
    .transparent(true)
    .always_on_top(true)
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
pub fn alert_log_client_error(app: AppHandle, title: String, message: String, error: String) {
    app.app_handle().alert(
        title.as_str(),
        message.as_str(),
        Some(anyhow::Error::msg(error)),
        false,
    );
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
