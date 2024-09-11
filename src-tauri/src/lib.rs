use tauri::Manager;

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
        window.close().unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_fullscreen(true).unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,answer,close_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
