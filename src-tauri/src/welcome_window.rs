use crate::alert::Alert;
use crate::app_config::AppConfig;
use crate::model::device::DeviceId;
use crate::model::settings::{SettingsUserDetails, WelcomeWizardMode};
use crate::model::welcome::WelcomeSettings;
use crate::settings_manager::{UserSettingsStore};
use crate::{tracking, tray, CountdownTimerState, SettingsManagerState, SubscriptionManagerState, TrackingState};
use log::{info, warn};
use std::time::Duration;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::{AppHandle, Manager, State};

const WINDOW_LABEL: &str = "welcome";

pub fn show(
    app: &AppHandle,
    device_id: &DeviceId,
    welcome_mode: WelcomeWizardMode,
) -> Result<(), anyhow::Error> {
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.set_focus()?;
        return Ok(());
    }

    let _window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App(format!("/welcome?mode={:?}", welcome_mode).into()),
    )
    .title("Welcome to Drink Now!")
    .center()
    .transparent(true)
    .focused(true)
    .decorations(false)
    .resizable(false)
    .shadow(true)
    .skip_taskbar(false)
    .visible(false)
    .build()?;

    if welcome_mode == WelcomeWizardMode::Complete {
        app.state::<TrackingState>()
            .send_tracking(tracking::Event::Install);
        open_thank_you(device_id);
    } else {
        app.state::<TrackingState>()
            .send_tracking(tracking::Event::ResetSettings);
    }

    Ok(())
}

#[specta::specta]
#[tauri::command]
pub fn welcome_load_settings(
    settings_manager: State<'_, SettingsManagerState>,
) -> Option<SettingsUserDetails> {
    settings_manager.get_settings().map(|s| s.user)
}

#[specta::specta]
#[tauri::command]
pub fn welcome_redo(app: AppHandle, tracking: State<TrackingState>) {
    info!("start welcome redo");
    show(
        app.app_handle(),
        &tracking.device_id(),
        WelcomeWizardMode::OnlySipSettings,
    )
    .unwrap_or_else(|err| {
        app.alert(
            "Unable to reset",
            "I am sorry, we are unable to reset the settings. Please try again later.",
            Some(err),
            false,
        )
    })
}

#[specta::specta]
#[tauri::command]
pub fn welcome_save(
    app: AppHandle,
    email: Option<String>,
    consent: Option<bool>,
    settings: WelcomeSettings,
    settings_manager: State<SettingsManagerState>,
    subscription_manager: State<SubscriptionManagerState>,
) {

    tray::show_tray_icon(app.app_handle());

    if let Some(consent) = consent {
        subscription_manager
            .subscribe(email, consent)
            .unwrap_or_else(|err| {
                app.alert(
                    "Can't subscribe",
                    "There was an error while subscribing",
                    Some(err),
                    true,
                )
            });
    }

    let current_settings = settings_manager
        .get_settings()
        .unwrap_or(UserSettingsStore::default())
        .user;

    // save settings
    settings_manager.update_user(SettingsUserDetails {
        next_break_duration_minutes: settings.next_break_duration_minutes,
        drink_amount_ml: settings.drink_amount_ml,
        sip_size: settings.sip_size,
        character: settings.character,
        gender_type: settings.gender_type,
        consent: consent.unwrap_or(current_settings.consent),
        ..current_settings
    }).unwrap_or_else(|err|
        app.alert(
            "Error while saving",
            "I am sorry, I am unable to save your settings. Please contact Rocket Solutions for support.",
            Some(err),
            false)
    );
}

#[specta::specta]
#[tauri::command]
pub fn welcome_close(
    app: AppHandle,
    settings_manager: State<SettingsManagerState>,
    timer: State<'_, CountdownTimerState>,
) {
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.destroy().expect("welcome window should be visible");
    }

    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Accessory)
        .expect("should allow to start app as accessory");

    match settings_manager.get_settings() {
        None => {
            warn!("no settings saved, can't start timer")
        }
        Some(s) => {
            timer.start(Duration::from_secs(
                (s.user.next_break_duration_minutes * 60) as u64,
            ));
        }
    }
}

pub fn open_thank_you(device_id: &DeviceId) {
    if cfg!(feature = "fullversion") {
        // apple does not allow cross-reference
    } else {
        let url = format!(
            "{}/thank-you/{}?utm_source=app&utm_medium=install",
            AppConfig::build().get_url(),
            device_id.get_hash_hex_id()
        );
        match webbrowser::open(url.as_str()) {
            Ok(_) => {}
            Err(err) => {
                warn!("can't open thank you page with browser: {}", err);
            }
        }
    }
}
