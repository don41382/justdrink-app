mod countdown_timer;
mod detect_mouse_state;
mod menubar;
mod model;
mod pretty_time;
mod session_repository;
mod session_window;
mod settings_window;
mod start_soon_window;
mod tray;
mod welcome_window;

use log::{error, warn};
#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use std::sync::Mutex;
use std::time::Duration;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::countdown_timer::CountdownTimer;
use crate::model::session::SessionDetail;
use crate::model::settings::SettingsDetails;
use crate::session_repository::SessionRepository;

#[cfg(target_os = "windows")]
use tauri::PhysicalPosition;

use tauri::{App, AppHandle, Manager, State, Window, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::Target;
use tauri_specta::{collect_commands, collect_events, Builder, Commands, Events};

#[specta::specta]
#[tauri::command]
fn update_settings(app_handle: AppHandle, settings: SettingsDetails) -> Result<(), String> {
    settings_window::set_settings(&app_handle, settings, true).map_err(|err| {
        error!("error while trying to save settings: {:?}", err);
        format!("error during update settings: {:?}", err)
    })?;
    Ok(())
}

#[specta::specta]
#[tauri::command]
fn start_first_session(app_handle: AppHandle, window: Window, next_break_duration_minutes: u32, enable_on_startup: bool) -> Result<(), String> {
    settings_window::set_settings(&app_handle, SettingsDetails {
        active: true,
        next_break_duration_minutes,
        enable_on_startup,
    }, true).map_err(|err| {
        error!("error while trying to save settings: {:?}", err);
        format!("error during update settings: {:?}", err)
    })?;
    window.hide().map_err(|err| {
        error!("error while closing welcome window: {:?}", err);
        format!("could not close window: {:?}", err)
    })?;
    session_window::start(&app_handle).map_err(|err| {
        error!("error while starting session: {:?}", err);
        format!("could not start session: {:?}", err)
    })?;
    Ok(())
}

#[specta::specta]
#[tauri::command]
fn close_window(window: Window, timer: State<CountdownTimer>) {
    timer.restart();
    window.close().unwrap();
}

#[specta::specta]
#[tauri::command]
fn load_session_details(session_repository: State<Mutex<SessionRepository>>) -> Result<SessionDetail, String> {
    {
        let mut repo = session_repository.lock().unwrap();
        match repo.pick_random_session() {
            None => {
                error!("no session found - this should not happen");
                Err("could not find a session".to_string())
            }
            Some(session) => {
                Ok(session.clone())
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = build_typescript_interfaces(
        collect_commands![close_window, update_settings, start_first_session, load_session_details,],
        collect_events![
            model::event::SessionStartEvent,
            model::event::SettingsEvent,
            countdown_timer::CountdownEvent,
            countdown_timer::CountdownStatus,
        ],
    )
        .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(tauri_plugin_log::TargetKind::Stdout),
                    Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level(log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .manage(Mutex::new(SessionRepository::new()))
        .setup(move |app| {
            builder.mount_events(app.app_handle());

            app.manage(CountdownTimer::new(app.app_handle()));

            match settings_window::get_settings(app.app_handle()) {
                Ok(settings) => {
                    app.manage(Mutex::new(Some(settings.clone())));
                    setup_timer(app, settings.clone()).unwrap();
                }
                Err(err) => {
                    warn!("could not load settings: {}", err);
                    welcome_window::show(app.app_handle())?;
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
            settings.next_break_duration_minutes as u64,
        ));
    }
    Ok(())
}
