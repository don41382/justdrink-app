mod menubar;
mod tray;
mod session_window;
mod session_timer;
mod pretty_time;

use std::sync::{Arc, Mutex};
use std::time::Duration;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use crate::session_timer::SessionTimer;
use tauri::{App, Manager, State, WebviewWindow, WindowEvent};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!!", name)
}

#[tauri::command]
fn answer(number: i32) -> String {
    number.to_string()
}

#[tauri::command]
fn close_app(app_handle: tauri::AppHandle, state: State<TimerState>) {
    session_window::close(&app_handle).unwrap();
    state.0.lock().unwrap().start(Duration::from_secs(30 * 60));
}

struct TimerState(Arc<Mutex<SessionTimer>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(TimerState(Arc::new(Mutex::new(SessionTimer::new()))))
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            session_window::new(app)?;
            tray::create_tray(app.handle())?;

            setup_timer(app).unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, answer, close_app])
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
            WindowEvent::Resized(size) => {
                log::info!("size x:{}, y:{}", size.width, size.height);
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
            log::info!("ended!");
        },
    );

    timer.start(Duration::from_secs(30 * 61));
    Ok(())
}