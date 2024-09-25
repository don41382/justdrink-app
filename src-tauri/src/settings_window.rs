use crate::countdown_timer::CountdownTimer;
use crate::model::settings::SettingsDetails;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;
use log::info;
use tauri::utils::config::WindowEffectsConfig;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Wry};
use tauri_plugin_store::{with_store, StoreCollection};
use crate::tracking;
use crate::tracking::Tracking;

pub(crate) const WINDOW_LABEL: &'static str = "settings";

const STORE_NAME: &str = "mm-config.json";

const DEFAULT_SESSION: SettingsDetails = SettingsDetails {
    next_break_duration_minutes: 120,
    active: true,
    allow_tracking: true,
    enable_on_startup: true,
};

#[specta::specta]
#[tauri::command]
pub fn load_settings_details(settings: State<Mutex<Option<SettingsDetails>>>) -> SettingsDetails {
    settings.lock().unwrap().clone().unwrap_or(DEFAULT_SESSION)
}

fn write_settings(
    app: &AppHandle,
    settings_details: &SettingsDetails,
) -> Result<(), anyhow::Error> {
    let stores = app
        .app_handle()
        .try_state::<StoreCollection<Wry>>()
        .unwrap();
    let path = PathBuf::from(STORE_NAME);
    with_store(app.app_handle().clone(), stores, path, |store| {
        let json_data = serde_json::to_value(settings_details)
            .map_err(|e| tauri_plugin_store::Error::Serialize(Box::new(e)))?;
        store.insert("data".to_string(), json_data)?;
        store.save()?;
        Ok(())
    })?;
    Ok(())
}

fn load_settings(app: &AppHandle) -> Result<SettingsDetails, anyhow::Error> {
    let stores = app
        .app_handle()
        .try_state::<StoreCollection<Wry>>()
        .unwrap();
    let path = PathBuf::from(STORE_NAME);
    info!("loading settings: {:?}",app.path().app_data_dir()?.join(&path));
    let details = with_store(app.app_handle().clone(), stores, path, |store| {
        let data_json = store
            .get("data".to_string())
            .ok_or_else(|| tauri_plugin_store::Error::NotFound(PathBuf::from("data")))?;

        let settings: SettingsDetails = serde_json::from_value(data_json.clone())
            .map_err(|e| tauri_plugin_store::Error::Deserialize(Box::new(e)))?;

        Ok(settings)
    })?;
    Ok(details)
}

pub fn set_settings(
    app: &AppHandle,
    settings: SettingsDetails,
    time_start: bool,
) -> Result<(), anyhow::Error> {
    let timer = app.state::<CountdownTimer>();
    {
        let settings_session = app.state::<Mutex<Option<SettingsDetails>>>();
        *settings_session.lock().unwrap() = Some(settings.clone());
    }

    // save settings
    write_settings(&app, &settings)?;

    // send tracking event
    app.state::<Tracking>().send_tracking(tracking::Event::SetTimer(settings.next_break_duration_minutes));

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

pub fn get_settings(app_handle: &AppHandle) -> Result<SettingsDetails, anyhow::Error> {
    load_settings(app_handle)
}

fn new<R>(app: &AppHandle<R>) -> Result<WebviewWindow<R>, anyhow::Error>
where
    R: Runtime,
{
    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/settings".into()),
    )
        .title("Settings")
        .inner_size(800.0, 600.0)
        .center()
        .visible(false)
        .always_on_top(true)
        .transparent(true)
        .decorations(true)
        .skip_taskbar(false)
        .effects(WindowEffectsConfig::default())
        .resizable(false)
        .build()?;
    Ok(window)
}

pub fn show<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    let _window = app.get_webview_window(WINDOW_LABEL).unwrap_or({
        new(app.app_handle())?
    });

    // the window wil show itself, once it's mounted

    Ok(())
}
