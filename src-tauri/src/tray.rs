use crate::countdown_timer::{CountdownEvent, CountdownStatus};
use crate::pretty_time::PrettyTime;
use crate::{session_window, settings_window};
use std::time::Duration;
use tauri::menu::{IconMenuItem, PredefinedMenuItem};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, Runtime,
};
use tauri_specta::Event;

const TRAY_ID: &'static str = "tray";

pub fn create_tray<R: Runtime>(main_app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let settings_menu = IconMenuItem::with_id(
        main_app,
        "settings",
        "Settings...",
        true,
        None,
        None::<&str>,
    )?;
    let quit_menu = MenuItem::with_id(main_app, "quit", "Quit", true, None::<&str>)?;
    let session_start_menu =
        MenuItem::with_id(main_app, "start", "Start session", true, None::<&str>)?;

    let separator = PredefinedMenuItem::separator(main_app)?;

    let menu = Menu::with_items(
        main_app,
        &[
            &session_start_menu,
            &separator,
            &settings_menu,
            &separator,
            &quit_menu,
        ],
    )?;

    let _ = TrayIconBuilder::with_id(TRAY_ID)
        .icon(main_app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "start" => {
                session_window::start(app.app_handle()).unwrap();
            }
            "settings" => {
                settings_window::show(app).unwrap();
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(main_app);

    let app_handle_tray_update = main_app.clone();
    CountdownEvent::listen(main_app.app_handle(), move |event| {
        let update: Option<Duration> = match event.payload.status {
            CountdownStatus::Start => {
                None
            }
            CountdownStatus::RunningSeconds { countdown_seconds } => {
                Some(Duration::from_secs(countdown_seconds as u64))
            }
            CountdownStatus::Finished => {
                None
            }
        };

        update_tray_title(
            &app_handle_tray_update,
            update,
        ).map_err(|e| log::error!("Failed to update tray title: {}", e))
            .ok();
    });

    Ok(())
}

pub fn update_tray_title<R: Runtime>(
    app_handle: &tauri::AppHandle<R>,
    duration: Option<Duration>,
) -> tauri::Result<()> {
    if let Some(tray) = app_handle.tray_by_id(TRAY_ID) {
        match duration {
            None => {
                tray.set_title::<String>(None)?;
            }
            Some(duration) => {
                tray.set_title(Some(duration.to_pretty_time()))?;
            }
        }
    }
    Ok(())
}
