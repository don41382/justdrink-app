use crate::alert::Alert;
use crate::app_config::AppConfig;
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
    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Regular)
        .expect("switch to regular app");

    for (_name, window) in app.webview_windows().iter() {
        if window.is_visible()? && window.label() != WINDOW_LABEL {
            window.hide()?;
        }
    }

    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let _window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App(format!("/welcome?mode={:?}", welcome_mode).into()),
    )
    .title("Welcome to Just Drink!")
    .center()
    .transparent(true)
    .focused(true)
    .decorations(false)
    .resizable(false)
    .shadow(true)
    .skip_taskbar(false)
    .visible(false)
    .build()?;

    let event = match welcome_mode {
        WelcomeWizardMode::Complete => tracking::Event::Install,
        WelcomeWizardMode::OnlySipSettings => tracking::Event::ResetSettings,
        WelcomeWizardMode::OnlyPayment => tracking::Event::OnlyPayment,
        WelcomeWizardMode::CancelPayment => tracking::Event::CancelPayment,
    };

    app.state::<TrackingState>()
        .send_tracking(event)
        .await;

    Ok(())
}

#[specta::specta]
#[tauri::command]
pub async fn welcome_with(app: AppHandle, welcome_wizard_mode: WelcomeWizardMode) {
    welcome_window::show(
        app.app_handle(),
        &app.state::<TrackingState>().device_id(),
        welcome_wizard_mode,
    )
    .await
    .expect("should show welcome window for payment")
}

#[specta::specta]
#[tauri::command]
pub fn welcome_load_settings(
    settings_manager: State<'_, SettingsManagerState>,
    tracking: State<'_, TrackingState>,
) -> WelcomeLoadSettings {
    WelcomeLoadSettings {
        user: settings_manager.get_settings().map(|s| s.user),
        backend_url: AppConfig::build().get_url(),
        device_id: tracking.device_id().get_hash_hex_id(),
    }
}

#[specta::specta]
#[tauri::command]
pub async fn welcome_redo(
    app: AppHandle,
    tracking: State<'_, TrackingState>,
) -> Result<(), String> {
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
pub async fn welcome_save(
    app: AppHandle,
    email: Option<String>,
    consent: Option<bool>,
    settings: WelcomeUserSettings,
    settings_manager: State<'_, SettingsManagerState>,
    subscription_manager: State<'_, SubscriptionManagerState>,
    timer: State<'_, CountdownTimerState>,
) -> Result<(), String> {
    tray::show_tray_icon(app.app_handle());

    if let Some(consent) = consent {
        subscription_manager
            .subscribe(email, consent)
            .await
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
    Ok(())
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

    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.destroy().expect("welcome window should be visible");
    }

    Ok(())
}

#[specta::specta]
#[tauri::command]
pub fn open_payment(app: AppHandle, tracking: State<'_, TrackingState>) {
    let url = format!(
        "{baseUrl}/pricing/checkout/justdrink/{deviceId}",
        baseUrl = AppConfig::build().get_url(),
        deviceId = tracking.device_id().get_hash_hex_id()
    );
    match webbrowser::open(url.as_str()) {
        Ok(_) => {}
        Err(err) => {
            app.alert("Unable to redirect to payment site", "Please ensure that your default browser is working. We are experience problems, opening the default browser", Some(anyhow!(err)), false);
        }
    }
}
