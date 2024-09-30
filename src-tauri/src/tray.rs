use crate::countdown_timer::{CountdownEvent, CountdownTimer, PauseOrigin, TimerStatus};
use crate::pretty_time::PrettyTime;
use crate::{alert, session_window, settings_window};
use std::time::Duration;
use anyhow::{anyhow};
use tauri::menu::{IconMenuItem, PredefinedMenuItem, Submenu};
use tauri::{menu::{Menu, MenuItem}, tray::TrayIconBuilder, Manager, Runtime};
use tauri_specta::Event;

const TRAY_ID: &'static str = "tray";

pub fn create_tray<R, M>(main_app: &M) -> tauri::Result<()>
where
    R: Runtime,
    M: Manager<R>,
{
    let timer = main_app.state::<CountdownTimer>();

    let menu_status = MenuItem::with_id(main_app, "status", timer.timer_status().to_text(), false, None::<&str>)?;
    let menu_timer_control = MenuItem::with_id(main_app, "timer_control", "Initializing", true, None::<&str>)?;

    let menu = Menu::with_items(
        main_app,
        &[
            &menu_status,
            &PredefinedMenuItem::separator(main_app)?,
            &Submenu::with_items(main_app, "Session", true, &[
                &MenuItem::with_id(main_app, "start", "Start now ...", true, None::<&str>)?,
                &menu_timer_control,
            ])?,
            &IconMenuItem::with_id(main_app, "settings", "Settings...", true, None, None::<&str>)?,
            &PredefinedMenuItem::separator(main_app)?,
            &MenuItem::with_id(main_app, "quit", "Quit", true, None::<&str>)?,
        ],
    )?;

    CountdownEvent::listen(main_app.app_handle(), move |event| {
        let timer_control_text = if event.payload.status.is_running() { "Pause" } else { "Resume" };

        menu_timer_control.set_text(timer_control_text).unwrap();
        menu_status.set_text(event.payload.status.to_text()).unwrap()
    });

    let _ = TrayIconBuilder::with_id(TRAY_ID)
        .icon(main_app.app_handle().default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "start" => {
                session_window::start(app.app_handle()).unwrap_or_else(|e| {
                    alert::alert(app, "Error while starting the session", "I am sorry, we are unable to start the session.", Some(e), false);
                });
            }
            "settings" => {
                settings_window::show(app).unwrap_or_else(|e| {
                    alert::alert(app, "Error while opening settings", "I am sorry, we are unable to open up the settings.", Some(anyhow!(e)), false);
                });
            }
            "timer_control" => {
                let timer = app.state::<CountdownTimer>();
                if timer.timer_status().is_running() {
                    timer.pause(PauseOrigin::User);
                } else {
                    timer.resume();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(main_app);

    let app_handle_tray_update = main_app.app_handle().clone();
    CountdownEvent::listen(main_app.app_handle(), move |event| {
        update_tray_title(&app_handle_tray_update, event.payload.status)
            .map_err(|e| log::error!("Failed to update tray title: {}", e))
            .ok();
    });

    Ok(())
}

pub fn update_tray_title<R, M>(
    app_handle: &M,
    status: TimerStatus,
) -> tauri::Result<()> where
    R: Runtime,
    M: Manager<R>,
{
    if let Some(tray) = app_handle.app_handle().tray_by_id(TRAY_ID) {
        let tray_text = match status {
            TimerStatus::NotStarted => None,
            TimerStatus::Active(duration) => Some(Duration::from_secs(duration as u64).to_pretty_time()),
            TimerStatus::Paused(origin) => match origin {
                PauseOrigin::Idle => Some("Idle".to_string()),
                PauseOrigin::User => Some("Paused".to_string())
            },
            TimerStatus::Finished => None,
        };
        tray.set_title(tray_text)?;
    }
    Ok(())
}
