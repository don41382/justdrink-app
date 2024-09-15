use crate::menubar::set_persistent_presentation_mode;
use crate::model::event::SessionStartEvent;
use crate::model::session::SessionDetail;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use tauri::{App, AppHandle, Manager, WebviewWindow};
use tauri_specta::Event;

const WINDOW_LABEL: &'static str = "session";

pub fn new(app: &mut App) -> Result<WebviewWindow, String> {
    set_persistent_presentation_mode(true);

    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/session".into()),
    )
    .title("Motion Minute Session")
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

pub fn show<R>(app: &AppHandle<R>, session: &SessionDetail) -> Result<(), String>
where
    R: tauri::Runtime,
{
    match app.get_webview_window(WINDOW_LABEL) {
        None => Err("Can't show session window, because it does not exist.".to_string()),
        Some(window) => {
            #[cfg(target_os = "macos")]
            app.app_handle()
                .set_activation_policy(ActivationPolicy::Regular)
                .unwrap();
            set_persistent_presentation_mode(true);

            SessionStartEvent {
                details: session.clone(),
            }
            .emit(&window.app_handle().clone())
            .unwrap();

            Ok(())
        }
    }
}
