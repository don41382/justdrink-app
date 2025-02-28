use std::error::Error;
use std::sync::Mutex;
use std::time::Duration;
use log::{info, warn};
use tauri::{ActivationPolicy, App, Manager};
use tauri_plugin_aptabase::EventTracker;
use tauri_specta::Builder;
use crate::{dashboard_window, detect_idling, feedback_window, license_manager, model, session_window, settings_system, show_dashboard, subscription_manager, tray, updater_window, welcome_window, CountdownTimerState, FeedbackSenderState, LicenseManagerState, SettingsManagerState, SettingsSystemState, SubscriptionManagerState, TrackingState};
use crate::countdown_timer::CountdownTimer;
use crate::model::settings::WelcomeWizardMode;
use crate::settings_manager::SettingsManager;
use crate::tracking::Tracking;

pub fn setup(builder: Builder, app: &mut App) -> Result<Result<(), Box<dyn Error>>, Box<dyn Error>> {
    app.track_event("app_started", None);
    builder.mount_events(app.app_handle());
    let device_id = model::device::DeviceId::lookup()?;
    info!(
                "application start, device id: {}",
                &device_id.get_hash_hex_id()
            );

    app.manage::<LicenseManagerState>(license_manager::LicenseManager::new(&device_id));
    app.manage::<FeedbackSenderState>(feedback_window::FeedbackSender::new(&device_id));
    app.manage::<SubscriptionManagerState>(subscription_manager::SubscriptionManager::new(
        device_id.clone(),
    ));

    let settings_manager = SettingsManager::new(app.app_handle())?;

    app.manage::<CountdownTimerState>(CountdownTimer::new(app.app_handle()));
    app.manage::<SettingsManagerState>(settings_manager);
    app.manage::<TrackingState>(Tracking::new(&device_id, app.app_handle())?);
    app.manage::<SettingsSystemState>(Mutex::new(settings_system::SettingsSystem::load(
        app.app_handle(),
    )));

    tray::create_tray(app.handle())?;

    match app.state::<SettingsManagerState>().get_settings() {
        Some(settings) => {
            tray::show_tray_icon(app.app_handle());
            if dashboard_window::should_show_dashboard() {
                show_dashboard(app.app_handle());
            }
            app.state::<CountdownTimerState>()
                .start(Duration::from_secs(
                    (settings.user.next_break_duration_minutes * 60) as u64,
                ));
            #[cfg(target_os = "macos")]
            app.app_handle()
                .set_activation_policy(ActivationPolicy::Accessory)
                .expect("should allow to start app as accessory");
        }
        None => {
            warn!("settings are missing, display welcome screen");
            let app = app.app_handle().clone();
            tauri::async_runtime::block_on(async move {
                welcome_window::show(app.app_handle(), &device_id, WelcomeWizardMode::Complete).await
            })?;
        }
    }

    session_window::init(app.app_handle())?;
    detect_idling::init(app.app_handle())?;

    let app_handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        info!("show updater window");
        tauri::async_runtime::spawn(async move {
            updater_window::show_if_update_available(&app_handle, true, true).await;
        });
    });

    Ok(Ok(()))
}