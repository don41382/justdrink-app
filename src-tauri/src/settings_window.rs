use crate::countdown_timer::CountdownTimer;
use crate::model::event::SettingsEvent;
use crate::model::settings::SettingsDetails;
use log::error;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;
use tauri::utils::config::WindowEffectsConfig;
use tauri::{AppHandle, Manager, WebviewWindow, Wry};
use tauri_plugin_store::{with_store, StoreCollection};
use tauri_specta::Event;

pub(crate) const WINDOW_LABEL: &'static str = "settings";

const STORE_NAME: &str = "config.bin";

const DEFAULT_SESSION: SettingsDetails = SettingsDetails {
    next_break_duration_minutes: 120,
    active: true,
};

fn write_settings(app: &AppHandle, settings_details: &SettingsDetails) -> Result<(), anyhow::Error> {
    let stores = app.app_handle().try_state::<StoreCollection<Wry>>().unwrap();
    let path = PathBuf::from(STORE_NAME);
    with_store(app.app_handle().clone(), stores, path, |store| {
        store.insert("active".to_string(), serde_json::Value::Bool(settings_details.active))?;
        store.insert("next_break_duration_minutes".to_string(), serde_json::Value::Number(settings_details.next_break_duration_minutes.into()))?;
        store.save()?;
        Ok(())
    })?;
    Ok(())
}

fn load_settings(app: &AppHandle) -> Result<SettingsDetails, anyhow::Error> {
    let stores = app.app_handle().try_state::<StoreCollection<Wry>>().unwrap();
    let path = PathBuf::from(STORE_NAME);
    let details = with_store(app.app_handle().clone(), stores, path, |store| {
        Ok(SettingsDetails {
            active: store.get("active".to_string()).map(|v| v.as_bool().unwrap()).unwrap_or(true),
            next_break_duration_minutes: store.get("next_break_duration_minutes".to_string()).map(|v| v.as_u64().unwrap() as u32).unwrap_or(180),
        })
    })?;
    Ok(details)
}

pub fn set_settings(app: &AppHandle, settings: SettingsDetails) -> Result<(), String> {
    let timer = app.state::<CountdownTimer>();
    {
        let settings_session = app.state::<Mutex<SettingsDetails>>();
        *settings_session.lock().unwrap() = settings.clone();
    }

    // save settings
    write_settings(&app, &settings)
        .map_err(|e| format!("error while writing settings: {}", e))?;

    // activate new settings
    if settings.active {
        timer.start(Duration::from_secs(
            settings.next_break_duration_minutes.into(),
        ));
    } else {
        timer.stop();
    }
    Ok(())
}

pub fn get_settings(app_handle: &AppHandle) -> SettingsDetails {
    load_settings(app_handle)
        .map_err(|e| error!("could not load settings: {:?}", e))
        .unwrap_or(DEFAULT_SESSION)
}

pub fn new(app: &AppHandle) -> Result<WebviewWindow, String> {
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
        .build()
        .map_err(|e| {
            log::error!("Failed to build WebviewWindow: {:?}", e);
            "Failed to build WebviewWindow".to_string()
        })?;

    Ok(window)
}

pub fn show<R>(app: &AppHandle<R>) -> Result<(), String>
where
    R: tauri::Runtime,
{
    match app.get_webview_window(WINDOW_LABEL) {
        None => Err("Can't show session window, because it does not exist.".to_string()),
        Some(window) => {
            let settings = app.state::<Mutex<SettingsDetails>>();
            SettingsEvent {
                details: settings.lock().unwrap().clone(),
            }
                .emit(&window.app_handle().clone())
                .unwrap();

            Ok(())
        }
    }
}
