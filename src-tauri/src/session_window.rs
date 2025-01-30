use log::{info};
use std::thread;
use std::thread::spawn;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::alert::Alert;
use crate::model::session::{ExerciseAvailability, SessionDetail};
use crate::model::settings::SettingsTabs;
use crate::{countdown_timer, feedback_window, fullscreen, model, settings_window, tracking, updater_window, CountdownTimerState, LicenseManagerState, SessionRepositoryState, SettingsSystemState, SubscriptionManagerState, TrackingState};
use tauri::{AppHandle, EventId, Manager, State, WebviewWindowBuilder, Window, Wry};
use tauri_specta::Event;

#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;
use crate::feedback_window::{FeedbackDisplay};
use crate::license_manager::{LicenseStatus, ValidTypes};

const WINDOW_LABEL: &'static str = "session";

pub fn init(app: &AppHandle<Wry>) -> EventId {
    let app_handle = app.clone();
    countdown_timer::CountdownEvent::listen(app, move |status| {
        if status.payload.status == countdown_timer::TimerStatus::Finished {
            let app_handle_start = app_handle.clone();
            app_handle.run_on_main_thread(move || {
                start(&app_handle_start.app_handle()).unwrap();
            }).unwrap();
        }
    })
}

// hack is required, if you remove async and run_mon_main_thread, it hangs on Windows machines
#[specta::specta]
#[tauri::command]
pub async fn start_session(app: AppHandle) -> () {
    app.clone().run_on_main_thread(move || {
        start(&app).unwrap_or_else(|err| {
            app.alert(
                "Can't start session",
                "There was an error while trying to start the session.",
                Some(err),
                false,
            );
            ()
        });
    }).expect("start_session running thread");
}

pub fn start(app: &AppHandle<Wry>) -> Result<(), anyhow::Error> {
    info!("Start session on thread: {}",
        thread::current()
            .name()
            .unwrap_or("Unnamed Thread")
            .to_string()
    );

    let license_manager = app.state::<LicenseManagerState>();
    if license_manager
        .try_lock()
        .expect("Could not lock license manager")
        .get_status(&app.app_handle(), false)
        .is_active()
    {
        // stop current running timer
        info!("start session window: stop timer");
        app.state::<CountdownTimerState>().stop();

        // stop current running timer
        info!("increase session counter");
        {
            let settings_system = app.state::<SettingsSystemState>();
            let mut settings_system = settings_system
                .lock()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            settings_system.increase_session_count(&app);
        }


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
                .title("Drink Now!")
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

        #[cfg(target_os = "windows")]
        apply_acrylic(&window, Some((18, 18, 18, 125)))
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");


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
    email: Option<String>,
    consent: bool,
    enable_on_startup: bool,
) -> Result<(), String> {
    spawn(move || {
        info!("starting first session");
        match start_first_session_(
            &app_handle.app_handle(),
            welcome_window,
            next_break_duration_minutes,
            email,
            consent,
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

fn start_first_session_(
    app_handle: &AppHandle,
    welcome_window: Window,
    next_break_duration_minutes: u32,
    email: Option<String>,
    consent: bool,
    enable_on_startup: bool,
) -> Result<(), anyhow::Error> {
    let subscription_manager = app_handle.state::<SubscriptionManagerState>();
    app_handle.state::<TrackingState>().send_tracking(tracking::Event::FirstSession);
    settings_window::set_settings(
        &app_handle,
        model::settings::SettingsUserDetails {
            active: true,
            next_break_duration_minutes,
            allow_tracking: true,
            enable_idle_detection: true,
            enable_on_startup,
            consent,
            beta_version: false,
        },
        true,
    )?;

    let status = app_handle.state::<LicenseManagerState>()
        .lock()
        .expect("require license manager")
        .get_status(app_handle.app_handle(), false);


    welcome_window.hide()?;

    match status {
        LicenseStatus::Valid(_) => {
            subscription_manager.subscribe(email.clone(), consent).unwrap_or_else(|err| {
                app_handle.alert("Unable to subscribe", format!("Sorry, we were not able to subscribe the user {:?}.", email.clone()).as_str(), Some(err), true);
            });
            start(&app_handle)?;
        }
        LicenseStatus::Expired(_) => {
            settings_window::show(app_handle, SettingsTabs::License)?
        }
        LicenseStatus::Invalid(_) => {
            settings_window::show(app_handle, SettingsTabs::License)?
        }
    }

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

    let status = license_manager
        .lock()
        .expect("license manager is locked")
        .get_status(app.app_handle(), false);

    let mut repo = session_repository.lock().unwrap();

    match repo.pick_random_session(&status.to_availability()) {
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
            Some(SessionDetail {
                exercise: exercise.clone(),
                license_info: status.to_license_info(),
            })
        }
    }
}

impl LicenseStatus {
    fn to_availability(&self) -> ExerciseAvailability {
        match &self {
            LicenseStatus::Valid(license) => {
                match license {
                    ValidTypes::Trial(_) => {
                        ExerciseAvailability::Trial
                    }
                    ValidTypes::Paid(_) => {
                        ExerciseAvailability::Full
                    }
                    ValidTypes::Full => {
                        ExerciseAvailability::Full
                    }
                }
            }
            _ => ExerciseAvailability::Trial
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
pub async fn end_session(
    app: AppHandle,
    window: Window,
    timer: State<'_, CountdownTimerState>,
    settings_system: State<'_, SettingsSystemState>,
    reason: model::session::SessionEndingReason,
    tracking: State<'_, TrackingState>,
) -> Result<(), String> {
    timer.restart();
    tracking.send_tracking(crate::tracking::Event::EndSession(reason));

    window.close().map_err(|err| format!("window can't be closed: {}", err))?;

    let ask_for_feedback = {
        let ss = settings_system.lock().expect("settings_system should not be locked");
        ss.should_show_feedback()
    };

    let updater_visible =
        updater_window::show_if_update_available(&app, false, false)
            .await;

    if ask_for_feedback && !updater_visible {
        feedback_window::show(&app).expect("unable to show feedback window");
    }

    Ok(())
}
