use std::time::Duration;
use crate::alert::Alert;
use crate::app_config::AppConfig;
use crate::model::device::DeviceId;
use crate::model::settings::SettingsUserDetails;
use crate::{model, session_window, settings_window, tracking, CountdownTimerState, SettingsManagerState, SubscriptionManagerState, TrackingState};
use log::warn;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::{AppHandle, Manager, State, Window};
use crate::subscription_manager::SubscriptionManager;

const WINDOW_LABEL: &str = "welcome";

pub fn show(app: &AppHandle, device_id: &DeviceId) -> Result<(), anyhow::Error> {
    let _window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/welcome".into()),
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

    app.state::<TrackingState>()
        .send_tracking(tracking::Event::Install);
    open_thank_you(device_id);

    Ok(())
}

#[specta::specta]
#[tauri::command]
pub fn welcome_finish(
    app: AppHandle,
    welcome_window: Window,
    email: Option<String>,
    settings: SettingsUserDetails,
    settings_manager: State<SettingsManagerState>,
    timer: State<'_, CountdownTimerState>,
    subscription_manager: State<SubscriptionManagerState>,
) {
    // hide welcome
    welcome_window
        .hide()
        .expect("welcome window should be visible");

    subscription_manager.subscribe(email, settings.consent).unwrap_or_else(|err|
        app.alert("Can't subscribe", "There was an error while subscribing", Some(err), true)
    );

    // switch to Accessory mode
    #[cfg(target_os = "macos")]
    app.app_handle()
        .set_activation_policy(ActivationPolicy::Accessory)
        .expect("should allow to start app as accessory");

    timer.start(Duration::from_secs((settings.next_break_duration_minutes * 60) as u64));

    // save settings
    settings_manager.update_user(settings).unwrap_or_else(|err|
            app.alert(
                "Error while saving",
                "I am sorry, I am unable to save your settings. Please contact Rocket Solutions for support.",
                      Some(err),
                false)
        );
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
