use crate::menubar::set_persistent_presentation_mode;
use crate::model::event::SessionStartEvent;
use crate::model::session::SessionDetail;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use tauri::{App, AppHandle, Manager, WebviewWindow};
use tauri_specta::Event;

pub const WINDOW_LABEL: &'static str = "start_soon";


pub fn show<R>(app: &AppHandle<R>) -> Result<(), String>
where
    R: tauri::Runtime,
{
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.close().map_err(|e| "Failed to close window".to_string())?;
    }

    tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/startsoon".into()),
    )
        .transparent(true)
        .visible(true)
        .always_on_top(true)
        .decorations(false)
        .skip_taskbar(true)
        .accept_first_mouse(true)
        .inner_size(200.0, 50.0)
        .shadow(false)
        .build()
        .map_err(|e| {
            log::error!("Failed to build WebviewWindow: {:?}", e);
            "Failed to build WebviewWindow".to_string()
        })?;

    Ok(())
}
