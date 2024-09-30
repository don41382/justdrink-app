use crate::countdown_timer::{TimerStatus};
use crate::{countdown_timer, detect_mouse_state, settings_window, CountdownTimerState};
use mouse_position::mouse_position::Mouse;
use std::thread::sleep;
use std::time::Duration;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};

#[cfg(target_os = "windows")]
use tauri::PhysicalPosition;

#[cfg(target_os = "macos")]
use tauri::LogicalPosition;

use tauri_specta::Event;

pub const WINDOW_LABEL: &'static str = "start_soon";

pub fn init<R>(mut app: &AppHandle<R>) -> Result<(), String>
where
    R: Runtime,
{
    let _ = new_window(app);

    glue_window_to_mouse(&mut app);

    let app_handle_show = app.clone();
    countdown_timer::CountdownEvent::listen(app, move |event| {
        let should_show_countdown = match event.payload.status {
            TimerStatus::Active(countdown) => {
                countdown > 0
                    && countdown < 6
                    && !app_handle_show
                    .get_webview_window(settings_window::WINDOW_LABEL)
                    .map(|w| w.is_visible().unwrap_or(false))
                    .unwrap_or(false)
            }
            _ => false,
        };
        if should_show_countdown {
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
    detect_mouse_state::detect_mouse_shake(Box::new(move |_| {
        if app_handle_shake.get_webview_window(WINDOW_LABEL).map(|w| w.is_visible().unwrap_or(false)).unwrap_or(false) {
            let timer = app_handle_shake.state::<CountdownTimerState>();
            timer.restart();
        }
    }));

    let app_handle_idle = app.app_handle().clone();
    detect_mouse_state::detect_mouse_idl(
        500,
        5
        , Box::new(move |mode| {
            match mode {
                detect_mouse_state::Mode::Idle => {
                    app_handle_idle.state::<CountdownTimerState>().pause(countdown_timer::PauseOrigin::Idle);
                }
                detect_mouse_state::Mode::Working => {
                    app_handle_idle.state::<CountdownTimerState>().resume();
                }
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
                            #[cfg(target_os = "windows")]
                            window
                                .set_position(PhysicalPosition::new(x + 20, y - 130))
                                .unwrap();
                            #[cfg(target_os = "macos")]
                            window
                                .set_position(LogicalPosition::new(x + 10, y - 90))
                                .unwrap();
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
        .shadow(false)
        .build()
        .map_err(|e| {
            log::error!("Failed to build WebviewWindow: {:?}", e);
            "Failed to build WebviewWindow".to_string()
        })?;
    Ok(window)
}
