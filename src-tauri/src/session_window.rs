use crate::menubar::set_persistent_presentation_mode;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use tauri::{AppHandle, EventId, Manager, WebviewWindowBuilder};
use tauri_specta::Event;
use crate::countdown_timer;

const WINDOW_LABEL: &'static str = "session";

pub fn init<R>(app: &AppHandle<R>) -> EventId
where
    R: tauri::Runtime + 'static,
{
    let app_handle = app.clone();
    countdown_timer::EventTickerStatus::listen(app, move |status| {
        if status.payload.status == countdown_timer::TickerStatus::FINISHED {
            println!("show it: {:?}", status.payload);
            show(&app_handle).unwrap();
        }
    })
}

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

