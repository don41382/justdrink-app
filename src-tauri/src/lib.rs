mod menubar;
mod tray;
mod session_window;
mod pretty_time;
mod model;
mod session_repository;
mod settings_window;
mod coutndown_timer;

use log::info;
#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::coutndown_timer::CountdownTimer;
use crate::model::settings::Settings;
use crate::session_repository::SessionRepository;
use tauri::{App, Manager, State, Window, WindowEvent};
use tauri_plugin_log::Target;
use tauri_specta::{collect_commands, collect_events, Builder, Commands, Events};

#[specta::specta]
#[tauri::command]
fn update_settings(settings: Settings, state: State<TimerState>, store_settings: State<Mutex<Settings>>) {
    let mut session_timer = state.0.lock().unwrap();
    *store_settings.lock().unwrap() = settings.clone();
    if settings.active {
        session_timer.start(Duration::from_secs(settings.next_break_duration_minutes.into()));
    } else {
        session_timer.stop();
    }
}


#[specta::specta]
#[tauri::command]
fn close_window(window: Window, state: State<TimerState>) {
    window.close().unwrap();
}

struct TimerState(Arc<Mutex<CountdownTimer>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = build_typescript_interfaces(
        collect_commands![close_window, update_settings,],
        collect_events![model::event::SessionStartEvent, model::event::SettingsEvent, ],
    ).unwrap();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(tauri_plugin_log::TargetKind::Stdout),
                    Target::new(tauri_plugin_log::TargetKind::Webview)]
                )
                .level(log::LevelFilter::Info)
                .build()
        )
        .invoke_handler(builder.invoke_handler())
        .manage(TimerState(Arc::new(Mutex::new(CountdownTimer::new()))))
        .manage(SessionRepository::new())
        .manage(Mutex::new(Settings {
            active: false,
            next_break_duration_minutes: 30,
        }))
        .setup(move |app| {
            builder.mount_events(app.app_handle());

            session_window::new(app)?;
            settings_window::new(app)?;

            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            tray::create_tray(app.handle())?;

            setup_timer(app).unwrap();

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                #[cfg(target_os = "macos")]
                window.app_handle().set_activation_policy(ActivationPolicy::Accessory).unwrap();
                api.prevent_close();
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
    builder
        .export(Typescript::default(), "../src/bindings.ts")?;

    Ok(builder)
}

fn setup_timer(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    let timer_state = app.state::<TimerState>();
    let mut timer = timer_state.0.lock().map_err(|e| format!("Failed to lock timer state: {}", e))?;

    timer.set_tick_callback(Box::new({
        let app_handle = app_handle.clone();
        move |time| {
            tray::update_tray_title(&app_handle, time)
                .map_err(|e| log::error!("Failed to update tray title: {}", e))
                .ok();
        }
    }));

    timer.set_finish_callback(Box::new({
        let app_handle = app_handle.clone();
        move || {
            let session_repository = app_handle.state::<SessionRepository>();
            if let Some(session) = session_repository.pick_random_session() {
                session_window::show(&app_handle, session)
                    .map_err(|e| log::error!("Failed to show session window: {}", e))
                    .ok();
            }
        }
    }));

    Ok(())
}