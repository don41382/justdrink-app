use crate::menubar::set_persistent_presentation_mode;
use crate::model::event::SessionStartEvent;
use crate::model::session::SessionDetail;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use tauri::{App, AppHandle, Manager, WebviewWindow};
use tauri_specta::Event;

const WINDOW_LABEL: &'static str = "session";


pub fn show<R>(app: &AppHandle<R>, session: &SessionDetail) -> Result<(), String>
where
    R: tauri::Runtime,
{
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.close().map_err(|e| "Failed to close window".to_string())?;
    }

    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Regular)
        .unwrap();
    set_persistent_presentation_mode(true);

    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/session".into()),
    )
        .title("Motion Minute Session")
        .visible(true)
        .always_on_top(true)
        .decorations(false)
        .skip_taskbar(true)
        .resizable(false)
        .fullscreen(true)
        .build()
        .map_err(|e| {
            log::error!("Failed to build WebviewWindow: {:?}", e);
            "Failed to build WebviewWindow".to_string()
        })?;

    Ok(())
}
