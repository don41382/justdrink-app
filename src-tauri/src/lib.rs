mod menubar;
mod tray;
mod session_window;
mod session_timer;
mod pretty_time;
mod model;

use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(debug_assertions)]
use specta_typescript::Typescript;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::session_timer::SessionTimer;
use tauri::{App, Manager, State, WindowEvent};
use tauri_plugin_log::Target;
use tauri_specta::{collect_commands, collect_events, Builder, Commands, Events};

#[specta::specta]
#[tauri::command]
fn close_app(app_handle: tauri::AppHandle, state: State<TimerState>) {
    session_window::close(&app_handle).unwrap();
    state.0.lock().unwrap().start(Duration::from_secs(30 * 60));
}

struct TimerState(Arc<Mutex<SessionTimer>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = build_typescript_interfaces(
        collect_commands![close_app,],
        collect_events![model::event::SessionStart,],
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
        .manage(TimerState(Arc::new(Mutex::new(SessionTimer::new()))))
        .setup(move |app| {
            builder.mount_events(app.app_handle());
            session_window::new(app)?;
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
        // Then register them (separated by a comma)
        .events(events)
        .commands(commands);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")?;

    Ok(builder)
}

fn setup_timer(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle_tick = app.handle().clone();
    let app_handle_end = app.handle().clone();
    let timer_state = app.state::<TimerState>();
    let mut timer = timer_state.0.lock().map_err(|e| format!("Failed to lock timer state: {}", e))?;

    timer.register(
        move |time| {
            tray::update_tray_title(&app_handle_tick, time)
                .map_err(|e| log::error!("Failed to update tray title: {}", e))
                .ok();
        },
        move || {
            session_window::show(&app_handle_end)
                .map_err(|e| log::error!("Failed to show session window: {}", e))
                .ok();
        },
    );

    timer.start(Duration::from_secs(5));
    Ok(())
}