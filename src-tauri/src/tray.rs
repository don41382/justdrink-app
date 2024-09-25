use crate::countdown_timer::{CountdownEvent, CountdownTimer, PauseOrigin, TimerStatus};
use crate::pretty_time::PrettyTime;
use crate::{alert, countdown_timer, session_window, settings_window};
use std::time::Duration;
use anyhow::{anyhow};
use tauri::menu::{IconMenuItem, PredefinedMenuItem, Submenu};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, Runtime,
};
use tauri::tray::TrayIconEvent;
use tauri_specta::Event;

const TRAY_ID: &'static str = "tray";

fn generate_menu<R, M>(main_app: &M) -> Result<Menu<R>, anyhow::Error>
where
    M: Manager<R>,
    R: Runtime,
{
    let timer = main_app.state::<CountdownTimer>();

    let status_string = match timer.timer_status() {
        TimerStatus::Active(duration) => {
            format!("Next session starts in {}", Duration::from_secs(duration as u64).to_pretty_time())
        }
        TimerStatus::Paused(origin) => {
            match origin {
                PauseOrigin::Idle =>
                    "Paused due to not interaction".to_string(),
                PauseOrigin::User =>
                    "Next session is paused".to_string()
            }

        }
        TimerStatus::NotStarted => {
            "Not running".to_string()
        }
        TimerStatus::Finished => {
            "Not running".to_string()
        }
    };

    let menu = Menu::with_items(
        main_app,
        &[
            &MenuItem::with_id(main_app, "status", status_string, false, None::<&str>)?,
            &PredefinedMenuItem::separator(main_app)?,
            &Submenu::with_items(main_app, "Session", true, &[
                &MenuItem::with_id(main_app, "start", "Start now ...", true, None::<&str>)?,
                &if timer.timer_status().is_running() {
                    MenuItem::with_id(main_app, "pause", "Pause", true, None::<&str>)?
                } else {
                    MenuItem::with_id(main_app, "resume", "Resume", true, None::<&str>)?
                }
            ])?,
            &IconMenuItem::with_id(main_app, "settings", "Settings...", true, None, None::<&str>)?,
            &PredefinedMenuItem::separator(main_app)?,
            &MenuItem::with_id(main_app, "quit", "Quit", true, None::<&str>)?,
        ],
    )?;
    Ok(menu)
}

pub fn create_tray<R, M>(main_app: &M) -> tauri::Result<()>
where
    R: Runtime,
    M: Manager<R>,
{
    let _ = TrayIconBuilder::with_id(TRAY_ID)
        .icon(main_app.app_handle().default_window_icon().unwrap().clone())
        .on_tray_icon_event({
            move |tray, event| {
                match event {
                    TrayIconEvent::Click { .. } => {}
                    TrayIconEvent::DoubleClick { .. } => {}
                    TrayIconEvent::Enter { .. } => {
                        match generate_menu(tray.app_handle())
                            .and_then(|menu|
                                Ok(tray.set_menu::<Menu<R>>(Some(menu))?)
                            ) {
                            Ok(_) => {}
                            Err(error) => {
                                alert(tray.app_handle(), "Menu not accessible", "I am sorry, there is an error while opening the menu item in the tray.", Some(error));
                            }
                        }
                    }
                    TrayIconEvent::Move { .. } => {}
                    TrayIconEvent::Leave { .. } => {}
                    _ => {}
                }
            }
        })
        .menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "start" => {
                session_window::start(app.app_handle()).unwrap_or_else(|e| {
                    alert(app, "Error while starting the session", "I am sorry, we are unable to start the session.", Some(e));
                });
            }
            "settings" => {
                settings_window::show(app).unwrap_or_else(|e| {
                    alert(app, "Error while opening settings", "I am sorry, we are unable to open up the settings.", Some(anyhow!(e)));
                });
            }
            "pause" => {
                app.state::<CountdownTimer>().pause(countdown_timer::PauseOrigin::User);
            }
            "resume" => {
                app.state::<CountdownTimer>().resume();
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
