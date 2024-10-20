use log::info;
use std::thread;
use std::thread::spawn;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::{alert, countdown_timer, fullscreen, model, start_first_session_, tracking, updater_window, CountdownTimerState, LicenseManagerState, SessionRepositoryState, TrackingState};
use tauri::{AppHandle, EventId, Manager, State, WebviewWindowBuilder, Window};
use tauri::webview::PageLoadEvent;
use tauri_specta::Event;
use crate::alert::Alert;
use crate::license_manager::{LicenseStatus, ValidTypes};
use crate::model::session::{Exercise, SessionDetail};

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

            Some(to_license_info(exercise, status))
        }
    }
}

fn to_license_info(exercise: &Exercise, status: LicenseStatus) -> SessionDetail {
    SessionDetail {
        exercise: exercise.clone(),
        license_info: match status {
            LicenseStatus::Valid(trailType) => {
                match trailType {
                    ValidTypes::Trail(details) => {
                        model::session::LicenseInfo {
                            status: model::session::LicenseStatus::Trail,
                            message: Some(format!("You have {:?} days remaining", days_between(chrono::Utc::now(), details.expired_at))),
                        }
                    }
                    ValidTypes::Paid(_details) => {
                        model::session::LicenseInfo {
                            status: model::session::LicenseStatus::Paid,
                            message: None,
                        }
                    }
                }
            }
            LicenseStatus::Expired(_) => model::session::LicenseInfo {
                status: model::session::LicenseStatus::Invalid,
                message: Some("Your license has expired".to_string()),
            },
            LicenseStatus::Invalid(_) => model::session::LicenseInfo {
                status: model::session::LicenseStatus::Invalid,
                message: Some("There is an issue with your license.".to_string()),
            },
        },
    }
}

fn days_between(start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> i64 {
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
