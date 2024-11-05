use log::info;
use std::thread;
use std::thread::spawn;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::alert::Alert;
use crate::license_manager::{LicenseStatus, ValidTypes};
use crate::model::license::LicenseInfoStatus;
use crate::model::session::{Exercise, SessionDetail};
use crate::model::settings::SettingsTabs;
use crate::{
    alert, countdown_timer, fullscreen, model, settings_window, start_first_session_, tracking,
    updater_window, CountdownTimerState, LicenseManagerState, SessionRepositoryState,
    TrackingState,
};
use tauri::webview::PageLoadEvent;
use tauri::{AppHandle, EventId, Manager, State, WebviewWindowBuilder, Window, Wry};
use tauri_specta::Event;

const WINDOW_LABEL: &'static str = "session";

pub fn init(app: &AppHandle<Wry>) -> EventId {
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

#[specta::specta]
#[tauri::command]
pub async fn start_session(app: AppHandle) -> () {
    start(&app)
        .unwrap_or_else(|err| {
            app.alert("Can't start session", "There was an error while trying to start the session.", Some(err), false);
            ()
        });
}

pub fn start(app: &AppHandle<Wry>) -> Result<(), anyhow::Error> {
    let license_manager = app.state::<LicenseManagerState>();
    if license_manager
        .try_lock()
        .expect("Could not lock license manager")
        .get_status()
        .is_active()
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
    } else {
        settings_window::show(app, SettingsTabs::License)?
    }

    Ok(())
}

#[specta::specta]
#[tauri::command]
pub async fn start_first_session(
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
            Err(error) => app_handle.alert(
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
    license_manager: State<LicenseManagerState>,
) -> Option<model::session::SessionDetail> {
    info!("load session details");

    let mut repo = session_repository.lock().unwrap();
    match repo.pick_random_session() {
        None => {
            app.alert(
                "Session is missing",
                "There is no session available",
                None,
                false,
            );
            None
        }
        Some(exercise) => {
            let status = license_manager
                .lock()
                .expect("license manager is locked")
                .get_status();

            Some(SessionDetail {
                exercise: exercise.clone(),
                license_info: status.to_license_info(),
            })
        }
    }
}

pub(crate) fn days_between(
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> i64 {
    let duration: chrono::Duration = end - start;
    duration.num_days()
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
