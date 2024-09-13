use crate::menubar::set_persistent_presentation_mode;
use crate::model::event::{SessionStartEvent, SettingsEvent};
use crate::model::session::SessionDetail;
use log::info;
use tauri::utils::config::WindowEffectsConfig;
use tauri::{ActivationPolicy, App, AppHandle, Manager, WebviewWindow};
use tauri_specta::Event;

const WINDOW_LABEL: &'static str = "settings";


pub fn new(app: &mut App) -> Result<WebviewWindow, String> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/settings".into()),
    )
        .title("Motion Minute Settings")
        .inner_size(800.0, 600.0)
        .visible(false)
        .always_on_top(true)
        .transparent(true)
        .decorations(false)
        .skip_taskbar(true)
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
            #[cfg(target_os = "macos")]
            app.app_handle().set_activation_policy(ActivationPolicy::Regular).unwrap();

            info!("open settings ...");

            SettingsEvent {
                name: "Felix".to_string()
            }.emit(&window.app_handle().clone()).unwrap();

            Ok(())
        }
    }
}