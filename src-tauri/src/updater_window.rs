use anyhow::Error;
use log::debug;
use tauri::{AppHandle, Manager, Runtime, Window};
use tauri_plugin_updater::UpdaterExt;
use crate::{alert, settings_system, SettingsSystemState};
use crate::settings_system::SettingsSystem;

const WINDOW_LABEL: &str = "updater";

pub fn show_if_update_available<R>(app: &AppHandle<R>, ignore_check_duration: bool) -> ()
where
    R: Runtime,
{
    debug!("updater info checking ...");
    let app_handle = app.app_handle().clone();
    tauri::async_runtime::spawn(async move {
        match show_if_update_available_run(&app_handle, ignore_check_duration).await {
            Ok(_) => {
                debug!("Check update finished");
            }
            Err(err) => {
                alert::alert(&app_handle, "Can't show update dialog", "Error while show update dialog.", Some(err), false);
            }
        }
    });
}

async fn show_if_update_available_run<R>(app_handle: &AppHandle<R>, ignore_check_duration: bool) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    {
        let settings_system = app_handle.state::<SettingsSystemState>();
        let mut settings_system = settings_system.lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if ignore_check_duration {
            debug!("checking for updates, ignore last check duration");
        } else {
            if !settings_system.updater_check_needed() {
                debug!("no update check required, last check is still recent enough");
                return Ok(());
            } else {
                settings_system.set_last_check_date(app_handle)?;
                debug!("let's check for updates, it's been a while");
            }
        }
        drop(settings_system);
    }

    if check_for_update_available(app_handle.app_handle()).await? {
        debug!("found new version. show update dialog.");
        let _ = show(&app_handle)?;
    }

    Ok(())
}

pub fn show<R>(app_handle: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    let visible = app_handle.get_webview_window(WINDOW_LABEL).map(|w| w.is_visible().unwrap_or(false)).unwrap_or(false);
    if !visible {
        let _w = tauri::WebviewWindowBuilder::new(
            app_handle,
            WINDOW_LABEL,
            tauri::WebviewUrl::App("/updater".into()),
        )
            .title("New version is available")
            .center()
            .visible(true)
            .auto_resize()
            .resizable(false)
            .always_on_top(true)
            .transparent(true)
            .decorations(false)
            .skip_taskbar(true)
            .resizable(true)
            .transparent(true)
            .shadow(false)
            .build()?;
    }

    Ok(())
}

async fn check_for_update_available<R>(app: &AppHandle<R>) -> Result<bool, anyhow::Error>
where
    R: Runtime,
{
    if let Some(_update) = app.updater()?.check().await? {
        Ok(true)
    } else {
        Ok(false)
    }
}


#[specta::specta]
#[tauri::command]
pub fn updater_close(
    app_handle: AppHandle,
    window: Window,
    restart: bool,
) {
    window.close().unwrap();
    if restart {
        app_handle.restart();
    }
}
