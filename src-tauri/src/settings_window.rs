use crate::alert::Alert;
use crate::model::settings::{SettingsTabs, SettingsUserDetails};
use crate::settings_manager::UserSettingsStore;
use crate::{
    model, tracking, CountdownTimerState, LicenseManagerState, SettingsManagerState, TrackingState,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::string::ToString;
use std::time::Duration;
use tauri::{AppHandle, Manager, Runtime, State, Window};
use tauri_plugin_store::StoreBuilder;

pub(crate) const WINDOW_LABEL: &'static str = "settings";

const STORE_NAME: &str = "mm-config.json";

#[specta::specta]
#[tauri::command]
pub async fn open_settings(app_handle: AppHandle) -> () {
    info!("open settings window");
    show(app_handle.app_handle(), SettingsTabs::Session).unwrap_or_else(|err| {
        app_handle.alert(
            "Can't open settings",
            "Error while opening settings. Please try again later.",
            Some(err),
            false,
        )
    });
}

#[specta::specta]
#[tauri::command]
pub fn load_settings(
    app: AppHandle,
    settings: State<'_, SettingsManagerState>,
) -> model::settings::Settings {
    info!("load settings data");
    let version = app.app_handle().config().version.clone();
    let license_manager = app.state::<LicenseManagerState>();
    let license_status = license_manager
        .lock()
        .unwrap()
        .get_status(&app.app_handle(), false);
    info!("load settings data - done");

    let settings = settings
        .get_settings()
        .unwrap_or(UserSettingsStore::default());

    model::settings::Settings {
        app: model::settings::AppDetails {
            version: version.unwrap_or("unknown".to_string()),
            license_info: license_status.to_license_info(),
        },
        user: settings.user,
        selected_tab: SettingsTabs::Session,
    }
}

#[specta::specta]
#[tauri::command]
pub fn update_settings(
    app_handle: AppHandle,
    settings: model::settings::SettingsUserDetails,
    settings_manager: State<SettingsManagerState>,
) -> () {
    //settings_manager.update_user().unwrap_or_else(|err| {
    //     app_handle.alert(
    //         "Failed to update settings",
    //         "Drink Now! is unable to update settings.",
    //         Some(err),
    //         false,
    //     );
    //     ()
    // });

    // if time_start {
    //     // activate new settings
    //     if settings.active {
    //         timer.start(Duration::from_secs(
    //             (settings.next_break_duration_minutes * 60).into(),
    //         ));
    //     } else {
    //         timer.stop();
    //     }
    // }
}

#[specta::specta]
#[tauri::command]
pub fn open_browser(window: Window, app_handle: AppHandle, url: String, close: bool) -> () {
    match webbrowser::open(url.as_str()) {
        Ok(_) => {}
        Err(err) => {
            let error = format!(
                "I am sorry, we are not able to open up the browser for '{}'",
                url
            );
            app_handle.alert(
                "Could not open Browser",
                error.as_str(),
                Some(anyhow::anyhow!(err)),
                false,
            );
        }
    }
    if close {
        window.close().expect("settings window to close");
    }
}

fn new<R>(
    app: &AppHandle<R>,
    selected_tab: model::settings::SettingsTabs,
) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    info!("start with new settings");
    let _window = tauri::WebviewWindowBuilder::new(
        app.app_handle(),
        WINDOW_LABEL,
        tauri::WebviewUrl::App(format!("/settings?settings_tab={:?}", selected_tab).into()),
    )
    .title("Settings")
    .center()
    .visible(false)
    .inner_size(1024.0, 768.0)
    .always_on_top(true)
    .transparent(true)
    .decorations(true)
    .skip_taskbar(false)
    .shadow(true)
    .minimizable(false)
    .maximizable(false)
    .resizable(false)
    .build()?;

    info!("start with new settings - done");
    Ok(())
}

pub fn show<R>(app: &AppHandle<R>, settings_tabs: SettingsTabs) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    info!("show settings: {:?}", settings_tabs);
    match app.get_webview_window(WINDOW_LABEL) {
        None => {
            info!("create new");
            new(app, settings_tabs)?;
        }
        Some(w) => {
            info!("show existing");
            w.show()?;
        }
    }
    Ok(())
}
