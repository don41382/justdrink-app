use std::thread;
use log::info;
use crate::menubar::set_persistent_presentation_mode;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::{countdown_timer, tracking};
use crate::countdown_timer::CountdownTimer;
use tauri::{AppHandle, EventId, Manager, WebviewWindowBuilder};
use tauri_specta::Event;

const WINDOW_LABEL: &'static str = "session";

pub fn init<R>(app: &AppHandle<R>) -> EventId
where
    R: tauri::Runtime + 'static,
{
    let app_handle = app.clone();
    countdown_timer::CountdownEvent::listen(app, move |status| {
        if status.payload.status == countdown_timer::TimerStatus::Finished {
            info!("Thread after timer runs out: {}", thread::current().name().unwrap_or("Unnamed Thread").to_string());
            start(&app_handle).unwrap();
        }
    })
}

pub fn start<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: tauri::Runtime,
{
    info!("start session window: stop timer");
    // stop current running timer
    app.state::<CountdownTimer>().stop();

    info!("start session window: send tracking");
    // send tracking event
    app.state::<tracking::Tracking>().send_tracking(tracking::Event::StartSession);

    info!("start session window: check window exists");
    if let Some(_window) = app.get_webview_window(WINDOW_LABEL) {
        info!("start session window: window already exists");
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Regular)?;

    set_persistent_presentation_mode(true);

    info!("start session window: create new window");
    let window =
        WebviewWindowBuilder::new(app, WINDOW_LABEL, tauri::WebviewUrl::App("/session".into()))
            .title("Motion Minute Session")
            .transparent(true)
            .visible(false)
            .always_on_top(true)
            .decorations(false)
            .maximized(true)
            .skip_taskbar(true)
            .resizable(false);

    #[cfg(target_os = "windows")]
    info!("start session window: fullscreen");
    #[cfg(target_os = "windows")]
    let window = window.fullscreen(true);

    info!("start session window: build");
    let window = window.build()?;

    info!("start session window: show window");
    window.show()?;

    Ok(())
}
