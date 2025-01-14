use crate::alert::Alert;
use crate::{SettingsDetailsState, SettingsSystemState};
use anyhow::Error;
use log::debug;
use tauri::{AppHandle, Manager, Runtime, Window};
use tauri::http::{HeaderMap, HeaderValue};
use tauri_plugin_updater::UpdaterExt;

const WINDOW_LABEL: &str = "updater";

pub async fn show_if_update_available(app: &AppHandle, skip_duration_check: bool, silent_error: bool) -> bool {
    debug!("updater info checking ...");
    let app_handle = app.app_handle().clone();
    let join = tauri::async_runtime::spawn(async move {
        match show_if_update_available_run(&app_handle, skip_duration_check).await {
            Ok(shown) => {
                debug!("Check update finished, dialog show: {shown}");
                shown
            }
            Err(err) => {
                app_handle.alert(
                    "Can't show update dialog",
                    "Error while show update dialog.",
                    Some(err),
                    silent_error,
                );
                false
            }
        }
    });
    join.await.expect("updater should be able to join thread")
}

async fn show_if_update_available_run<R>(
    app_handle: &AppHandle<R>,
    skip_duration_check: bool,
) -> Result<bool, anyhow::Error>
where
    R: Runtime,
{
    let shown = if !cfg!(feature = "fullversion") && check_for_new_version_required(app_handle.app_handle(), skip_duration_check)? {
        if is_new_version_available(app_handle.app_handle()).await? {
            debug!("found new version. show update dialog.");
            let _ = show(&app_handle)?;
            true
        } else {
            false
        }
    } else {
        false
    };

    Ok(shown)
}

fn check_for_new_version_required<R: Runtime>(
    app_handle: &AppHandle<R>,
    skip_duration_check: bool,
) -> Result<bool, Error> {
    let settings_system = app_handle.state::<SettingsSystemState>();
    let mut settings_system = settings_system
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let check_for_updates = if skip_duration_check {
        debug!("checking for updates, ignore last check duration");
        true
    } else {
        if settings_system.updater_check_needed() {
            debug!("let's check for updates, it's been a while");
            true
        } else {
            debug!("no update check required, last check is still recent enough");
            false
        }
    };

    if check_for_updates {
        settings_system.set_last_check_date(app_handle)?;
    }

    Ok(check_for_updates)
}

pub fn show<R>(app_handle: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    let visible = app_handle
        .get_webview_window(WINDOW_LABEL)
        .map(|w| w.is_visible().unwrap_or(false))
        .unwrap_or(false);
    if !visible {
        let pre_release: bool = {
            let sds = app_handle.state::<SettingsDetailsState>();
            let sd =
                sds
                    .lock()
                    .expect("settings details can't be unlocked");
            sd.clone().map(|d| d.beta_version).unwrap_or(false)
        };

        let _w = tauri::WebviewWindowBuilder::new(
            app_handle,
            WINDOW_LABEL,
            tauri::WebviewUrl::App(format!("/updater?prerelease={:?}", pre_release).into()),
        )
            .title("New version is available")
            .resizable(false)
            .visible(false)
            .always_on_top(true)
            .transparent(true)
            .decorations(false)
            .skip_taskbar(true)
            .resizable(false)
            .transparent(true)
            .shadow(true)
            .build()?;
    }

    Ok(())
}

async fn is_new_version_available<R>(app: &AppHandle<R>) -> Result<bool, anyhow::Error>
where
    R: Runtime,
{
    let pre_release: bool = {
        let sds = app.state::<SettingsDetailsState>();
        let sd =
            sds
                .lock()
                .expect("settings details can't be unlocked");
        sd.clone().map(|d| d.beta_version).unwrap_or(false)
    };

    let mut headers = HeaderMap::new();
    headers.insert("prerelease", HeaderValue::from_str(pre_release.to_string().as_str())?);

    debug!("checking for new update, pre-release: {pre_release}");

    if let Some(_update) = app
        .updater_builder()
        .headers(headers)
        .build()
        ?.check().await? {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[specta::specta]
#[tauri::command]
pub fn updater_close(window: Window) {
    window.close().unwrap();
}
