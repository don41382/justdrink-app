mod menubar;
mod tray;

use crate::menubar::set_persistent_presentation_mode;
use tauri::{Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!!", name)
}

#[tauri::command]
fn answer(number: i32) -> String {
    number.to_string()
}

#[tauri::command]
fn close_app(app_handle: tauri::AppHandle) {
    if let Some(window) = app_handle.webview_windows().get("main") {
        window.close().unwrap()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        //.enable_macos_default_menu(false)
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            set_persistent_presentation_mode(true);

            let window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
                .visible(false)
                //.always_on_top(true)
                .decorations(false)
                .skip_taskbar(true)
                .maximized(true)
                .resizable(true)
                .build()
                .unwrap();


            let handle = app.handle();
            tray::create_tray(handle)?;

            let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
                    if shortcut == &ctrl_n_shortcut {
                        match event.state() {
                            ShortcutState::Pressed => {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                            ShortcutState::Released => {}
                        }
                    }
                })
                    .build(),
            )?;

            app.global_shortcut().register(ctrl_n_shortcut)?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, answer, close_app])
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
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
