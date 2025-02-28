use crate::alert::Alert;
use crate::app_config::AppConfig;
use crate::license_manager::LicenseManager;
use crate::model::device::DeviceId;
use crate::model::settings::{SettingsUserDetails, WelcomeWizardMode};
use crate::model::welcome::{WelcomeLoadSettings, WelcomeUserSettings};
use crate::settings_manager::{SettingsManager, UserSettingsStore};
use crate::tracking::Event;
use crate::{
    dashboard_window, tracking, tray, welcome_window, CountdownTimerState, LicenseManagerState,
    SettingsManagerState, SubscriptionManagerState, TrackingState,
};
use anyhow::anyhow;
use log::{info, warn};
use std::time::Duration;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::{AppHandle, Manager, State};

const WINDOW_LABEL: &str = "welcome";

pub async fn show(
    app: &AppHandle,
    device_id: &DeviceId,
    welcome_mode: WelcomeWizardMode,
) -> Result<(), anyhow::Error> {
    for (_name, window) in app.webview_windows().iter() {
        if window.is_visible()? && window.label() != WINDOW_LABEL {
            window.hide()?;
        }
    }

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
            .send_tracking(tracking::Event::Install)
            .await;
        open_thank_you(device_id);
    } else {
        app.state::<TrackingState>()
            .send_tracking(tracking::Event::ResetSettings)
            .await;
    }

    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Regular)
        .expect("switch to regular app");

    Ok(())
}

#[specta::specta]
#[tauri::command]
pub async fn welcome_only_payment(app: AppHandle) {
    welcome_window::show(
        app.app_handle(),
        &app.state::<TrackingState>().device_id(),
        WelcomeWizardMode::OnlyPayment,
    )
    .await
    .expect("should show welcome window for payment")
}

#[specta::specta]
#[tauri::command]
pub fn welcome_load_settings(
    settings_manager: State<'_, SettingsManagerState>,
    tracking: State<'_, TrackingState>
) -> WelcomeLoadSettings {
    WelcomeLoadSettings {
        user: settings_manager.get_settings().map(|s| s.user),
        backend_url: AppConfig::build().get_url(),
        device_id: tracking.device_id().get_hash_hex_id(),
    }
}

#[specta::specta]
#[tauri::command]
pub async fn welcome_redo(app: AppHandle, tracking: State<'_, TrackingState>) -> Result<(), String> {
    info!("start welcome redo");
    show(
        app.app_handle(),
        &tracking.device_id(),
        WelcomeWizardMode::OnlySipSettings,
    )
    .await
    .unwrap_or_else(|err| {
        app.alert(
            "Unable to reset",
            "I am sorry, we are unable to reset the settings. Please try again later.",
            Some(err),
            false,
        )
    });
    Ok(())
}

#[specta::specta]
#[tauri::command]
pub fn welcome_save(
    app: AppHandle,
    email: Option<String>,
    consent: Option<bool>,
    settings: WelcomeUserSettings,
    settings_manager: State<SettingsManagerState>,
    subscription_manager: State<SubscriptionManagerState>,
    timer: State<'_, CountdownTimerState>,
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

#[specta::specta]
#[tauri::command]
pub async fn welcome_close(
    app: AppHandle,
    settings_manager: State<'_, SettingsManager>,
    license_manager_state: State<'_, LicenseManagerState>,
    tracking: State<'_, TrackingState>,
    state: String,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.destroy().expect("welcome window should be visible");
    }

    let _ = license_manager_state
        .refresh_license_status(app.app_handle())
        .await?;

    info!("close welcome, state: {}", state);

    tracking.send_tracking(Event::WelcomeQuit(state)).await;

    if let None = settings_manager.get_settings() {
        info!("quitting app, if configuration not finished yet.");
        app.exit(0);
    } else {
        dashboard_window::show(app.app_handle()).expect("should show dashboard after welcome");

        #[cfg(target_os = "macos")]
        app.app_handle()
            .set_activation_policy(ActivationPolicy::Accessory)
            .expect("switch back to accessory");
    }
    Ok(())
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
