mod countdown_timer;
mod detect_mouse_state;
mod menubar;
mod model;
mod pretty_time;
mod session_repository;
mod session_window;
mod settings_window;
mod start_soon_window;
mod tracking;
mod tray;
mod welcome_window;
mod alert;

use log::{info, warn};
#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use std::sync::{Mutex};
use std::thread::spawn;
use std::time::Duration;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::countdown_timer::CountdownTimer;
use crate::model::settings::SettingsDetails;
use crate::session_repository::SessionRepository;

use tauri::{App, AppHandle, Manager, State, Window, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::Target;
use tauri_specta::{collect_commands, collect_events, Builder, Commands, Events};
use crate::alert::alert;
use crate::tracking::{Event, Tracking};

#[specta::specta]
#[tauri::command]
fn update_settings(app_handle: AppHandle, settings: SettingsDetails) -> () {
    settings_window::set_settings(&app_handle, settings, true).unwrap_or_else(|err| {
        alert(app_handle.app_handle(), "Failed to update settings", "Motion minute is unable to update settings.", Some(err));
        ()
    });
}

#[specta::specta]
#[tauri::command]
fn start_first_session(
    app_handle: AppHandle,
    welcome_window: Window,
    next_break_duration_minutes: u32,
    enable_on_startup: bool,
) -> Result<(), String> {
    spawn(move || {
        info!("starting first session");
        match start_first_session_(&app_handle.app_handle(), welcome_window, next_break_duration_minutes, enable_on_startup) {
            Ok(_) => {}
            Err(error) => {
                alert(&app_handle.app_handle(), "Not able to start first session", format!("{:?}", error).as_str(), Some(error))
            }
        }
    });
    Ok(())
}

fn start_first_session_(app_handle: &AppHandle, welcome_window: Window, next_break_duration_minutes: u32, enable_on_startup: bool) -> Result<(), anyhow::Error> {
    settings_window::set_settings(
        &app_handle,
        SettingsDetails {
            active: true,
            next_break_duration_minutes,
            allow_tracking: true,
            enable_on_startup,
        },
        true,
    )?;
    welcome_window.hide()?;
    session_window::start(&app_handle)?;
    Ok(())
}

#[specta::specta]
#[tauri::command]
fn end_session(window: Window, timer: State<CountdownTimer>, reason: model::session::SessionEndingReason, tracking: State<Tracking>) {
    timer.restart();
    window.close().unwrap();
    tracking.send_tracking(Event::EndSession(reason));
}

#[specta::specta]
#[tauri::command]
fn load_session_details(
    app: AppHandle,
    session_repository: State<Mutex<SessionRepository>>,
) -> Option<model::session::SessionDetail> {
    {
        let mut repo = session_repository.lock().unwrap();
        match repo.pick_random_session() {
            None => {
                alert(&app, "Session is missing", "There is no session available", None);
                None
            }
            Some(session) => Some(session.clone()),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = build_typescript_interfaces(
        collect_commands![
            end_session,
            update_settings,
            start_first_session,
            load_session_details,
            alert::close_error_window,
        ],
        collect_events![
            model::event::SessionStartEvent,
            model::event::SettingsEvent,
            model::event::AlertEvent,
            model::session::SessionEndingReason,
            countdown_timer::CountdownEvent,
            countdown_timer::CountdownStatus,

        ],
    )
        .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(tauri_plugin_log::TargetKind::Stdout),
                    Target::new(tauri_plugin_log::TargetKind::Webview),
                    Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("motion-minutes".to_string())
                    })
                ])
                .level_for("tao::platform_impl::platform::window_delegate", log::LevelFilter::Info)
                .level(log::LevelFilter::Trace)
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .manage(Mutex::new(SessionRepository::new()))
        .setup(move |app| {
            builder.mount_events(app.app_handle());
            alert::init(app.app_handle())?;

            app.manage(CountdownTimer::new(app.app_handle()));
            app.manage(Tracking::new(app.app_handle()).unwrap());

            match settings_window::get_settings(app.app_handle()) {
                Ok(settings) => {
                    app.manage(Mutex::new(Some(settings.clone())));
                    setup_timer(app, settings.clone()).unwrap();
                }
                Err(err) => {
                    warn!("could not load settings: {}", err);
                    welcome_window::show(app.app_handle())?;
                    info!("display welcome screen");
                    app.manage(Mutex::new(None::<SettingsDetails>));
                }
            }

            session_window::init(app.app_handle());
            start_soon_window::init(app.app_handle())?;
            settings_window::new(app.app_handle())?;

            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            tray::create_tray(app.handle())?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { .. } => {
                #[cfg(target_os = "macos")]
                window
                    .app_handle()
                    .set_activation_policy(ActivationPolicy::Accessory)
                    .unwrap();

                window.hide().unwrap();
            }
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::DragDrop(_) => {}
            WindowEvent::ThemeChanged(_) => {}
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_typescript_interfaces(
    commands: Commands<tauri::Wry>,
    events: Events,
) -> Result<Builder, Box<dyn std::error::Error>> {
    let builder = Builder::<tauri::Wry>::new()
        .events(events)
        .commands(commands);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder.export(Typescript::default(), "../src/bindings.ts")?;

    Ok(builder)
}

fn setup_timer(app: &mut App, settings: SettingsDetails) -> Result<(), Box<dyn std::error::Error>> {
    let timer = app.state::<CountdownTimer>();

    if settings.active {
        timer.start(Duration::from_secs(
            (settings.next_break_duration_minutes * 60).into(),
        ));
    }
    Ok(())
}
