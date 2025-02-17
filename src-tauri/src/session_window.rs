use crate::alert::Alert;
use crate::model::settings::SettingsTabs;
use crate::{
    countdown_timer, feedback_window, model, settings_window, tracking, updater_window,
    CountdownTimerState, LicenseManagerState, SettingsSystemState, SubscriptionManagerState,
    TrackingState,
};
use anyhow::Error;
use core::clone::Clone;
use log::info;
use tauri::{AppHandle, EventId, Manager, State, WebviewWindowBuilder, Window, Wry};
use tauri_specta::Event;

use crate::feedback_window::FeedbackDisplay;
use crate::license_manager::LicenseStatus;
use crate::model::event::SessionStartEvent;
use crate::model::session::{DrinkCharacter, SipSize};

const WINDOW_LABEL: &'static str = "session";

pub fn init(app: &AppHandle<Wry>) -> Result<EventId, anyhow::Error> {
    let app_handle = app.clone();
    build_session_window(app)?;
    let id = countdown_timer::CountdownEvent::listen(app, move |status| {
        if status.payload.status == countdown_timer::TimerStatus::Finished {
            let app_handle_start = app_handle.clone();
            app_handle
                .run_on_main_thread(move || {
                    show_session(&app_handle_start.app_handle(), None).unwrap();
                })
                .unwrap();
        }
    });
    Ok(id)
}

#[specta::specta]
#[tauri::command]
pub fn start_session(
    app: AppHandle,
    drink_settings: Option<SessionStartEvent>,
) -> Result<(), ()> {
    show_session(&app, drink_settings).unwrap_or_else(|err| {
        app.alert(
            "Can't start session",
            "There was an error while trying to start the session.",
            Some(err),
            false,
        );
        ()
    });
    Ok(())
}

pub fn show_session(
    app: &AppHandle<Wry>,
    drink_settings: Option<SessionStartEvent>,
) -> Result<(), anyhow::Error> {
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

        // TODO: read from saved setting, if character is not explicitly named
        let drink_settings = drink_settings.unwrap_or_else(|| SessionStartEvent {
            sip_size: SipSize::BigSip,
            selected_drink_character: DrinkCharacter::YoungWoman,
        });

        if let Some(_window) = app.get_webview_window(WINDOW_LABEL) {
            info!("start session window: send event");
            drink_settings.emit(app.app_handle())?;
        } else {
            app.alert("Session Window Missing", "I am sorry, this should not happen. Please contact Rocket Solutions", None, false);
        }
    } else {
        settings_window::show(app, SettingsTabs::License)?
    }

    Ok(())
}

fn build_session_window(app: &AppHandle) -> Result<(), Error> {
    info!("start session window: create new window");
    let window =
        WebviewWindowBuilder::new(app, WINDOW_LABEL, tauri::WebviewUrl::App("/session".into()))
            .title("Drink Now! Session")
            .transparent(true)
            .visible(false)
            .always_on_top(true)
            .decorations(false)
            .maximized(true)
            .skip_taskbar(false)
            .accept_first_mouse(true)
            .focused(false)
            .resizable(false);

    info!("start session window: build");
    let window = window.build()?;
    window.set_ignore_cursor_events(true)?;

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
    app_handle
        .state::<TrackingState>()
        .send_tracking(tracking::Event::FirstSession);
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

    let status = app_handle
        .state::<LicenseManagerState>()
        .lock()
        .expect("require license manager")
        .get_status(app_handle.app_handle(), false);

    welcome_window.hide()?;

    match status {
        LicenseStatus::Valid(_) => {
            subscription_manager
                .subscribe(email.clone(), consent)
                .unwrap_or_else(|err| {
                    app_handle.alert(
                        "Unable to subscribe",
                        format!(
                            "Sorry, we were not able to subscribe the user {:?}.",
                            email.clone()
                        )
                        .as_str(),
                        Some(err),
                        true,
                    );
                });
            show_session(&app_handle, None)?;
        }
        LicenseStatus::Expired(_) => settings_window::show(app_handle, SettingsTabs::License)?,
        LicenseStatus::Invalid(_) => settings_window::show(app_handle, SettingsTabs::License)?,
    }

    Ok(())
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
) -> Result<(), String> {
    timer.restart();

    window
        .hide()
        .map_err(|err| format!("window can't be closed: {}", err))?;

    let ask_for_feedback = {
        let ss = settings_system
            .lock()
            .expect("settings_system should not be locked");
        ss.should_show_feedback()
    };

    let updater_visible = updater_window::show_if_update_available(&app, false, false).await;

    if ask_for_feedback && !updater_visible {
        feedback_window::show(&app).expect("unable to show feedback window");
    }

    Ok(())
}
