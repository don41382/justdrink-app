use anyhow::Error;
use log::error;
use tauri::{AppHandle, Manager, Runtime};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use tauri_specta::Event;
use crate::model::event::AlertEvent;

const WINDOW_LABEL: &str = "alert";


pub fn init<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: tauri::Runtime,
{
    let _ = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/alert".into()),
    )
        .title("Oops, we didn't expect this")
        .center()
        .inner_size(600.0, 300.0)
        .visible(false)
        .transparent(true)
        .always_on_top(false)
        .decorations(false)
        .skip_taskbar(true)
        .resizable(true)
        .shadow(false)
        .build()?;

    Ok(())
}

pub fn alert<R>(app: &AppHandle<R>, title: &str, message: &str, error: Option<anyhow::Error>, silence: bool) -> ()
where
    R: Runtime,
{
    if let Some(e) = error {
        error!("error '{}' with message '{}', error: {:?}", title, message, e);
    } else {
        error!("error '{}' with message '{}'", title, message);
    }

    if !silence {
        match display_alert(app, title, message) {
            Ok(_) => {}
            Err(e) => {
                error!("unable to display error {:?}", e);
                app.exit(-1);
            }
        }
    }
}

fn display_alert<R>(app: &AppHandle<R>, title: &str, message: &str) -> Result<(), Error>
where
    R: Runtime,
{
    #[cfg(target_os = "macos")]
    app.app_handle().set_activation_policy(ActivationPolicy::Regular)?;

    let alert_window = app.get_webview_window(WINDOW_LABEL).expect("alert window must exists");
    alert_window.show()?;

    AlertEvent {
        title: title.to_string(),
        message: message.to_string(),
    }.emit(alert_window.app_handle())?;

    Ok(())
}

#[specta::specta]
#[tauri::command]
pub fn close_error_window(window: tauri::Window) {
    window.hide().expect("alert window must exists and should never be closed");
}
