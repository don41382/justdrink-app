use crate::alert::Alert;
use crate::model::settings::{SettingsTabs, SettingsUserDetails};
use crate::{
    alert, model, tracking, CountdownTimerState, LicenseManagerState, SettingsDetailsState,
    TrackingState,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::string::ToString;
use std::time::Duration;
use tauri::utils::config::WindowEffectsConfig;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Window, WindowEvent};
use tauri_plugin_store::StoreBuilder;
use tauri_specta::Event;

pub(crate) const WINDOW_LABEL: &'static str = "settings";

const STORE_NAME: &str = "mm-config.json";

const DEFAULT_SESSION: SettingsUserDetails = SettingsUserDetails {
    next_break_duration_minutes: 60,
    active: true,
    allow_tracking: true,
    enable_on_startup: true,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserSettingsStore {
    version: String,
    details: SettingsUserDetails,
}

#[specta::specta]
#[tauri::command]
pub async fn open_settings(
    app_handle: AppHandle
) -> () {
    info!("open settings window");
    show(app_handle.app_handle(), SettingsTabs::Session)
        .unwrap_or_else(|err|
            app_handle.alert("Can't open settings", "Error while opening settings. Please try again later.", Some(err), false)
        );
}

#[specta::specta]
#[tauri::command]
pub fn load_settings(
    app_handle: AppHandle,
    settings: State<SettingsDetailsState>,
) -> model::settings::Settings {
    info!("load settings data");
    let version = app_handle.app_handle().config().version.clone();
    let license_manager = app_handle.state::<LicenseManagerState>();
    let license_status = license_manager.lock().unwrap().get_status();
    info!("load settings data - done");
    model::settings::Settings {
        app: model::settings::AppDetails {
            version: version.unwrap_or("unknown".to_string()),
            license_info: license_status.to_license_info(),
        },
        user: settings.lock().unwrap().clone().unwrap_or(DEFAULT_SESSION),
        selected_tab: SettingsTabs::Session,
    }
}

#[specta::specta]
#[tauri::command]
pub fn open_browser(window: Window, app_handle: AppHandle, url: String) -> () {
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
    window.close().expect("settings window to close");
}

fn write_settings(
    app: &AppHandle,
    settings_details: &SettingsUserDetails,
) -> Result<(), anyhow::Error> {
    let store = StoreBuilder::new(app.app_handle(), STORE_NAME).build();
    let version = app.app_handle().config().version.clone();

    let json_data = serde_json::to_value(UserSettingsStore {
        version: version.unwrap_or("0.0.0".to_string()),
        details: settings_details.clone(),
    })
        .map_err(|e| tauri_plugin_store::Error::Serialize(Box::new(e)))?;

    store.set("data".to_string(), json_data);
    store.save()?;
    Ok(())
}

fn load_settings_store(app: &AppHandle) -> Result<UserSettingsStore, anyhow::Error> {
    let path = PathBuf::from(STORE_NAME);
    info!(
        "loading settings: {:?}",
        app.path().app_data_dir()?.join(&path)
    );
    let store = StoreBuilder::new(app.app_handle(), STORE_NAME).build();

    let data_json = store
        .get("data".to_string())
        .ok_or_else(|| tauri_plugin_store::Error::NotFound(PathBuf::from("data")))?;

    let details: UserSettingsStore = serde_json::from_value(data_json.clone())
        .map_err(|e| tauri_plugin_store::Error::Deserialize(Box::new(e)))?;

    Ok(details)
}

pub fn set_settings(
    app: &AppHandle,
    settings: SettingsUserDetails,
    time_start: bool,
) -> Result<(), anyhow::Error> {
    let timer = app.state::<CountdownTimerState>();
    {
        let settings_session = app.state::<SettingsDetailsState>();
        *settings_session.lock().unwrap() = Some(settings.clone());
    }

    // save settings
    write_settings(&app, &settings)?;

    // send tracking event
    app.state::<TrackingState>()
        .send_tracking(tracking::Event::SetTimer(
            settings.next_break_duration_minutes,
        ));

    if time_start {
        // activate new settings
        if settings.active {
            timer.start(Duration::from_secs(
                (settings.next_break_duration_minutes * 60).into(),
            ));
        } else {
            timer.stop();
        }
    }
    Ok(())
}

pub fn get_settings(app_handle: &AppHandle) -> Result<SettingsUserDetails, anyhow::Error> {
    let settings = load_settings_store(app_handle)?;
    Ok(settings.details)
}

fn new<R>(
    app: &AppHandle<R>,
    selected_tab: model::settings::SettingsTabs,
) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    info!("start with new settings");
    let window = tauri::WebviewWindowBuilder::new(
        app.app_handle(),
        WINDOW_LABEL,
        tauri::WebviewUrl::App(format!("/settings?settings_tab={:?}", selected_tab).into()),
    )
        .title("Settings")
        .inner_size(800.0, 400.0)
        .center()
        .visible(false)
        .always_on_top(true)
        .transparent(true)
        .decorations(true)
        .skip_taskbar(false)
        .shadow(true)
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
