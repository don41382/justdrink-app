use crate::pretty_time::PrettyTime;
use crate::session_window;
use std::time::Duration;
use tauri::{menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, Runtime};

const TRAY_ID: &'static str = "tray";

pub fn create_tray<R: Runtime>(main_app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(main_app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(main_app, &[&quit_i])?;

    let _ = TrayIconBuilder::with_id(TRAY_ID)
        .icon(main_app.default_window_icon().unwrap().clone())
        .title("10min")
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            // Add more events here
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event {
                session_window::show(tray.app_handle()).unwrap();
            }
        })
        .build(main_app);

    Ok(())
}

pub fn update_tray_title<R: Runtime>(app_handle: &tauri::AppHandle<R>, duration: Duration) -> tauri::Result<()> {
    if let Some(tray) = app_handle.tray_by_id(TRAY_ID) {
        tray.set_title(Some(duration.to_pretty_time()))?;
    }
    Ok(())
}
