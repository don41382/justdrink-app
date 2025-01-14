use crate::alert::Alert;
use crate::countdown_timer::{CountdownEvent, CountdownTimer, PauseOrigin, TimerStatus};
use crate::model::settings::SettingsTabs;
use crate::pretty_time::PrettyTime;
use crate::{dashboard_window, feedback_window, session_window, settings_window, updater_window};
use anyhow::anyhow;
use std::time::Duration;
use tauri::menu::{IconMenuItem, PredefinedMenuItem, Submenu};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};
use tauri::image::Image;
use tauri::path::BaseDirectory;
use tauri_specta::Event;

const TRAY_ID: &'static str = "tray";

pub fn create_tray(main_app: &AppHandle<Wry>) -> tauri::Result<()> {
    let menu_status = MenuItem::with_id(main_app, "dashboard", "Dashboard", true, None::<&str>)?;
    let menu_timer_control = MenuItem::with_id(
        main_app,
        "timer_control",
        "Initializing",
        true,
        None::<&str>,
    )?;

    let menu = Menu::with_items(
        main_app,
        &[
            &menu_status,
            &PredefinedMenuItem::separator(main_app)?,
            &Submenu::with_items(
                main_app,
                "Session",
                true,
                &[
                    &MenuItem::with_id(main_app, "start", "Start now ...", true, None::<&str>)?,
                    &menu_timer_control,
                ],
            )?,
            &IconMenuItem::with_id(
                main_app,
                "settings",
                "Settings...",
                true,
                None,
                None::<&str>,
            )?,
            &PredefinedMenuItem::separator(main_app)?,
            &IconMenuItem::with_id(main_app, "about", "About ...", true, None, None::<&str>)?,
            #[cfg(not(feature = "fullversion"))]
            &IconMenuItem::with_id(
                main_app,
                "updater",
                "Check for updates ...",
                true,
                None,
                None::<&str>,
            )?,
            &IconMenuItem::with_id(
                main_app,
                "feedback",
                "Feedback ...",
                true,
                None,
                None::<&str>,
            )?,
            &PredefinedMenuItem::separator(main_app)?,
            &MenuItem::with_id(main_app, "quit", "Quit", true, None::<&str>)?,
        ],
    )?;

    let app_handle = main_app.clone();
    CountdownEvent::listen(main_app.app_handle(), move |event| {
        let timer_control_text = if event.payload.status.is_running() {
            "Pause"
        } else {
            "Resume"
        };

        menu_timer_control.set_text(timer_control_text).unwrap_or_else(|err | {
            app_handle.alert("Can't set timer in tray", "Unable to update tray", Some(anyhow::anyhow!(err)), true);
        });

        menu_status
            .set_text(format!("Dashboard ({})", event.payload.status.to_text()))
            .unwrap()
    });

    let tray_icon = tray_icon(main_app.app_handle())?;
    let _ = TrayIconBuilder::with_id(TRAY_ID)
        .icon(tray_icon)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "dashboard" => {
                dashboard_window::show(app.app_handle()).unwrap_or_else(|e| {
                    app.alert(
                        "Error while showing dashboard",
                        "I am sorry, we are unable to show the dashboard. Please try again later.",
                        Some(e),
                        false,
                    )
                });
            }
            "start" => {
                session_window::start(app.app_handle()).unwrap_or_else(|e| {
                    app.alert(
                        "Error while starting the session",
                        "I am sorry, we are unable to start the session.",
                        Some(e),
                        false,
                    );
                });
            }
            "settings" => {
                settings_window::show(app, SettingsTabs::Session).unwrap_or_else(|e| {
                    app.alert(
                        "Error while opening settings",
                        "I am sorry, we are unable to open up the settings.",
                        Some(anyhow!(e)),
                        false,
                    );
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
            #[cfg(not(feature = "fullversion"))]
            "updater" => {
                updater_window::show(app.app_handle()).unwrap_or_else(|e| {
                    app.alert(
                        "Error while opening updater",
                        "I am sorry, we are unable to open the updater.",
                        Some(anyhow!(e)),
                        false,
                    );
                });
            }
            "about" => {
                settings_window::show(app, SettingsTabs::About).unwrap_or_else(|e| {
                    app.alert(
                        "Error while opening settings",
                        "I am sorry, we are unable to open up the settings.",
                        Some(anyhow!(e)),
                        false,
                    );
                });
            },
            "feedback" => {
                feedback_window::show(app).unwrap_or_else(|e| {
                    app.alert(
                        "Error while opening settings",
                        "I am sorry, we are unable to open up feedback.",
                        Some(anyhow!(e)),
                        false,
                    );
                });
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(main_app)?;

    let app_handle_tray_update = main_app.app_handle().clone();
    CountdownEvent::listen(main_app.app_handle(), move |event| {
        update_tray_title(&app_handle_tray_update, event.payload.status)
            .map_err(|e| log::error!("Failed to update tray title: {}", e))
            .ok();
    });

    Ok(())
}

pub fn update_tray_title(app_handle: &AppHandle<Wry>, status: TimerStatus) -> tauri::Result<()> {
    if let Some(tray) = app_handle.tray_by_id(TRAY_ID) {
        let tray_text = match status {
            TimerStatus::NotStarted(_) => None,
            TimerStatus::Active(duration) => {
                Some(Duration::from_secs(duration as u64).to_pretty_time())
            }
            TimerStatus::Paused(origin, _) => match origin {
                PauseOrigin::Idle => Some("Idle".to_string()),
                PauseOrigin::PreventSleep(_) => Some("Busy".to_string()),
                PauseOrigin::User => Some("Paused".to_string()),
            },
            TimerStatus::Finished => None,
        };
        tray.set_title(tray_text)?;
    }
    Ok(())
}

fn tray_icon(app: &AppHandle<Wry>) -> tauri::Result<Image<'_>> {
    if cfg!(target_os = "macos") {
        let image = Image::from_path(app.path().resolve("icons/128x128-tray-light.png", BaseDirectory::Resource)?)?;
        Ok(image)
    } else {
        let image =
            app
                .default_window_icon()
                .ok_or_else(|| tauri::Error::Anyhow(anyhow!("tray icon is missing")))?;
        Ok(image.clone())
    }
}
