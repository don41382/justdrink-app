use crate::model::event::{SettingsEvent};
use crate::model::settings::SettingsDetails;
use std::sync::{Mutex};
use tauri::utils::config::WindowEffectsConfig;
use tauri::{App, AppHandle, Manager, WebviewWindow};
use tauri_specta::Event;

const WINDOW_LABEL: &'static str = "settings";

pub fn new(app: &mut App) -> Result<WebviewWindow, String> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/settings".into()),
    )
    .title("Settings")
    .inner_size(800.0, 600.0)
    .center()
    .visible(false)
    .always_on_top(true)
    .transparent(true)
    .decorations(true)
    .skip_taskbar(false)
    .effects(WindowEffectsConfig::default())
    .resizable(false)
    .build()
    .map_err(|e| {
        log::error!("Failed to build WebviewWindow: {:?}", e);
        "Failed to build WebviewWindow".to_string()
    })?;
    Ok(window)
}

pub fn show<R>(app: &AppHandle<R>) -> Result<(), String>
where
    R: tauri::Runtime,
{
    match app.get_webview_window(WINDOW_LABEL) {
        None => Err("Can't show session window, because it does not exist.".to_string()),
        Some(window) => {
            let settings = app.state::<Mutex<SettingsDetails>>();
            SettingsEvent {
                details: settings.lock().unwrap().clone(),
            }
            .emit(&window.app_handle().clone())
            .unwrap();

            Ok(())
        }
    }
}
