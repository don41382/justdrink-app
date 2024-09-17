use crate::menubar::set_persistent_presentation_mode;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use tauri::{AppHandle, Manager, WebviewWindowBuilder};

const WINDOW_LABEL: &'static str = "session";


pub fn show<R>(app: &AppHandle<R>) -> Result<(), String>
where
    R: tauri::Runtime,
{
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.close().map_err(|_| "Failed to close window".to_string())?;
    }

    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Regular)
        .unwrap();
    set_persistent_presentation_mode(true);

    let window = WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/session".into()),
    )
        .title("Motion Minute Session")
        .transparent(true)
        .visible(true)
        .always_on_top(true)
        .decorations(false)
        .maximized(true)
        .skip_taskbar(true)
        .resizable(false);

    #[cfg(target_os = "windows")]
    let window = window.fullscreen(true);

    window.build()
        .map_err(|e| {
            log::error!("Failed to build WebviewWindow: {:?}", e);
            "Failed to build WebviewWindow".to_string()
        })?;

    Ok(())
}

