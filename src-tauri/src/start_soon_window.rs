use crate::{countdown_timer, detect_mouse_state, TimerState};
use log::error;
use mouse_position::mouse_position::Mouse;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tauri::{App, AppHandle, Emitter, Listener, LogicalPosition, Manager, PhysicalPosition, Runtime, Size, WebviewWindow};
use tauri_specta::Event;

pub const WINDOW_LABEL: &'static str = "start_soon";


pub fn init<R>(mut app: &AppHandle<R>) -> Result<(), String>
where
    R: Runtime,
{
    let _ = new_window(app);

    glue_window_to_mouse(&mut app);

    let app_handle_show = app.clone();
    countdown_timer::EventTicker::listen(app, move |event| {
        if event.payload.countdown > 0 && event.payload.countdown < 10 {
            if let Some(window) = app_handle_show.get_webview_window(WINDOW_LABEL) {
                window.show().unwrap()
            }
        } else {
            if let Some(window) = app_handle_show.get_webview_window(WINDOW_LABEL) {
                if window.is_visible().unwrap() {
                    window.hide().unwrap()
                }
            }
        }
    });

    let app_handle_shake = app.app_handle().clone();
    detect_mouse_state::init(&mut app.app_handle(), Box::new(move |_| {
        if let Some(window) = app_handle_shake.get_webview_window(WINDOW_LABEL) {
            let timer = app_handle_shake.state::<TimerState>();
            timer.0.lock().unwrap().restart();
        }
    }));

    Ok(())
}

fn glue_window_to_mouse<R>(app: &mut &AppHandle<R>)
where
    R: Runtime,
{
    let app_mouse_position_handler = app.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            match app_mouse_position_handler.get_webview_window(WINDOW_LABEL) {
                None => {
                    sleep(Duration::from_millis(100));
                }
                Some(window) => {
                    let position = Mouse::get_mouse_position();
                    match position {
                        Mouse::Position { x, y } => {
                            let y_correction = 90;
                            #[cfg(target_os = "windows")]
                            window.set_position(PhysicalPosition::new(x + 10, y - y_correction)).unwrap();
                            #[cfg(target_os = "macos")]
                            window.set_position(LogicalPosition::new(x + 10, y - y_correction as i32)).unwrap();
                        }
                        Mouse::Error => {}
                    }
                    sleep(Duration::from_millis(5));
                }
            }
        }
    });
}

fn new_window<R>(app: &AppHandle<R>) -> Result<WebviewWindow<R>, String>
where
    R: Runtime,
{
    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/startsoon".into()),
    )
        .title("Start Soon Message")
        .center()
        .visible(false)
        .always_on_top(true)
        .transparent(true)
        .decorations(false)
        .skip_taskbar(true)
        .resizable(true)
        .transparent(true)
        .build()
        .map_err(|e| {
            log::error!("Failed to build WebviewWindow: {:?}", e);
            "Failed to build WebviewWindow".to_string()
        })?;
    Ok(window)
}
