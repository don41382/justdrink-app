use log::info;
use std::thread;
use std::thread::spawn;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::{alert, countdown_timer, fullscreen, model, start_first_session_, tracking, updater_window, CountdownTimerState, SessionRepositoryState, TrackingState};
use tauri::{AppHandle, EventId, Manager, State, WebviewWindowBuilder, Window};
use tauri_specta::Event;

const WINDOW_LABEL: &'static str = "session";

pub fn init<R>(app: &AppHandle<R>) -> EventId
where
    R: tauri::Runtime + 'static,
{
    let app_handle = app.clone();
    countdown_timer::CountdownEvent::listen(app, move |status| {
        if status.payload.status == countdown_timer::TimerStatus::Finished {
            info!(
                "Thread after timer runs out: {}",
                thread::current()
                    .name()
                    .unwrap_or("Unnamed Thread")
                    .to_string()
            );
            start(&app_handle).unwrap();
        }
    })
}

pub fn start<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: tauri::Runtime,
{
    // stop current running timer
    info!("start session window: stop timer");
    app.state::<CountdownTimerState>().stop();

    // send tracking event
    info!("start session window: send tracking");
    app.state::<TrackingState>()
        .send_tracking(tracking::Event::StartSession);

    info!("start session window: check window exists");
    if let Some(_window) = app.get_webview_window(WINDOW_LABEL) {
        info!("start session window: window already exists");
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Regular)?;

    fullscreen::enforce_full_screen(true);

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

#[specta::specta]
#[tauri::command]
pub fn start_first_session(
    app_handle: AppHandle,
    welcome_window: Window,
    next_break_duration_minutes: u32,
    enable_on_startup: bool,
) -> Result<(), String> {
    spawn(move || {
        info!("starting first session");
        match start_first_session_(
            &app_handle.app_handle(),
            welcome_window,
            next_break_duration_minutes,
            enable_on_startup,
        ) {
            Ok(_) => {}
            Err(error) => alert::alert(
                &app_handle.app_handle(),
                "Not able to start first session",
                format!("{:?}", error).as_str(),
                Some(error),
                false,
            ),
        }
    });
    Ok(())
}

#[specta::specta]
#[tauri::command]
pub fn load_session_details(
    app: AppHandle,
    session_repository: State<SessionRepositoryState>,
) -> Option<model::session::SessionDetail> {
    {
        let mut repo = session_repository.lock().unwrap();
        match repo.pick_random_session() {
            None => {
                alert::alert(
                    &app,
                    "Session is missing",
                    "There is no session available",
                    None,
                    false,
                );
                None
            }
            Some(session) => Some(session.clone()),
        }
    }
}

#[specta::specta]
#[tauri::command]
pub fn end_session(
    app: AppHandle,
    window: Window,
    timer: State<CountdownTimerState>,
    reason: model::session::SessionEndingReason,
    tracking: State<TrackingState>,
) {
    timer.restart();
    window.close().unwrap();
    tracking.send_tracking(crate::tracking::Event::EndSession(reason));

    updater_window::show_if_update_available(&app, false);
}
