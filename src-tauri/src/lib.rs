mod alert;
mod countdown_timer;
mod detect_idling;
mod model;
mod pretty_time;
mod tracking;
mod tray;

mod app_config;
mod dashboard_window;
mod feedback_window;
mod license_manager;
mod session_window;
mod settings_manager;
mod settings_system;
mod settings_window;
mod subscription_manager;
mod updater_window;
mod welcome_window;
mod setup;

use log::{info};
use serde_json::json;
#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use std::sync::Mutex;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::countdown_timer::CountdownTimer;

use crate::alert::Alert;
use crate::settings_manager::SettingsManager;
use crate::settings_system::SettingsSystem;
use crate::tracking::Tracking;
use tauri::{AppHandle, Manager, RunEvent, WindowEvent};
use tauri_plugin_aptabase::EventTracker;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::Target;
use tauri_specta::{collect_commands, collect_events, Builder, Commands, ErrorHandlingMode, Events};

type FeedbackSenderState = feedback_window::FeedbackSender;
type SettingsManagerState = SettingsManager;
type SettingsSystemState = Mutex<SettingsSystem>;
type CountdownTimerState = CountdownTimer;
type TrackingState = Tracking;
type LicenseManagerState = license_manager::LicenseManager;
type SubscriptionManagerState = subscription_manager::SubscriptionManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = build_typescript_interfaces(
        collect_commands![
            alert::alert_log_client_error,
            dashboard_window::get_current_timer_status,
            dashboard_window::toggle_timer,
            dashboard_window::timer_change,
            feedback_window::feedback_window_send_feedback,
            session_window::start_session,
            session_window::end_session,
            settings_window::open_settings,
            settings_window::load_settings,
            settings_window::get_device_id,
            settings_window::update_settings,
            settings_window::open_browser,
            welcome_window::welcome_only_payment,
            welcome_window::welcome_load_settings,
            welcome_window::welcome_redo,
            welcome_window::welcome_save,
            welcome_window::welcome_close,
            alert::close_error_window,
            updater_window::updater_close,
            license_manager::request_license_status,

        ],
        collect_events![
            model::event::SessionStartEvent,
            model::settings::Settings,
            model::settings::WelcomeWizardMode,
            model::settings::SettingsUserDetails,
            license_manager::LicenseResult,
            countdown_timer::CountdownEvent,
            countdown_timer::TimerStatus,
        ],
    )
    .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
            info!("open dashboard - only on windows");
            #[cfg(target_os = "windows")]
            {
                info!("instance of Drink Now! already open");
                show_dashboard(_app);
            }
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--quiet"]),
        ))
        .plugin(
            tauri_plugin_aptabase::Builder::new("A-EU-1597927502")
                .with_panic_hook(Box::new(|client, info, msg| {
                    info!("panic detection");
                    let location = info
                        .location()
                        .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
                        .unwrap_or_else(|| "".to_string());
                    client.track_event(
                        "panic",
                        Some(json!({
                            "info": format!("{} ({})", msg, location),
                        })),
                    );
                }))
                .build(),
        )
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(tauri_plugin_log::TargetKind::Stdout),
                    Target::new(tauri_plugin_log::TargetKind::Webview),
                    Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("drinknow".to_string()),
                    }),
                ])
                .level_for(
                    "tao::platform_impl::platform::window_delegate",
                    log::LevelFilter::Info,
                )
                .level_for("tao::platform_impl::platform::view", log::LevelFilter::Info)
                .level_for("tauri_plugin_aptabase::dispatcher", log::LevelFilter::Info)
                .level(log::LevelFilter::Trace)
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .enable_macos_default_menu(false)
        .setup(move |app| {
            setup::setup(builder, app)
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                #[cfg(target_os = "macos")]
                window
                    .app_handle()
                    .set_activation_policy(ActivationPolicy::Accessory)
                    .unwrap();

                window.hide().unwrap();
                api.prevent_close();
            }
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::DragDrop(_) => {}
            WindowEvent::ThemeChanged(_) => {}
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            #[cfg(target_os = "macos")]
            RunEvent::Reopen { .. } => {
                info!("Reopen Drink Now - Show Dashboard");
                // called only on macos
                show_dashboard(app);
            }
            RunEvent::ExitRequested { .. } => {
                info!("Closing Drink Now! Stop timer.");
                app.track_event("app_quit", None);
                let timer = app.state::<CountdownTimerState>();
                timer.stop();
            }
            _ => {}
        })
}


fn show_dashboard(app: &AppHandle) {
    dashboard_window::show(app.app_handle()).unwrap_or_else(|err| {
        app.alert(
            "Can't open action menu",
            "Action Menu can't be opened during new instance. Please try again later.",
            Some(err),
            false,
        );
    });
}

fn build_typescript_interfaces(
    commands: Commands<tauri::Wry>,
    events: Events,
) -> Result<Builder, Box<dyn std::error::Error>> {
    let builder = Builder::<tauri::Wry>::new()
        .error_handling(ErrorHandlingMode::Throw)
        .events(events)
        .commands(commands);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder.export(Typescript::default(), "../src/bindings.ts")?;

    Ok(builder)
}
