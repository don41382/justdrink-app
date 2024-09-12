use crate::menubar::set_persistent_presentation_mode;
use tauri::{ActivationPolicy, App, AppHandle, Manager, WebviewWindow};

const WINDOW_LABEL: &'static str = "main";

pub fn new(app: &mut App) -> Result<WebviewWindow, String> {
    set_persistent_presentation_mode(true);

    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("index.html".into()),
    )
        .visible(false)
        //.always_on_top(true)
        .decorations(false)
        .skip_taskbar(true)
        .maximized(true)
        .resizable(true)
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
            set_persistent_presentation_mode(true);
            window.show().unwrap();
            window.set_focus().unwrap();
            Ok(())
        }
    }
}

pub(crate) fn close(app_handle: &AppHandle) -> Result<(), String> {
    app_handle.get_webview_window(WINDOW_LABEL).map_or_else(
        || Err("window can't be closed".to_string()),
        |w| w.close().map_err(|_| "window can't be closed".to_string()),
    )
}