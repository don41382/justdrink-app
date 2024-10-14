use crate::model::settings::SettingsUserDetails;
use crate::{alert, model, tracking, CountdownTimerState, SettingsDetailsState, TrackingState};
use log::info;
use std::path::PathBuf;
use std::time::Duration;
use tauri::utils::config::WindowEffectsConfig;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Window, WindowEvent};
use tauri_plugin_store::{StoreBuilder};
use tauri_specta::Event;

pub(crate) const WINDOW_LABEL: &'static str = "settings";

const STORE_NAME: &str = "mm-config.json";

const DEFAULT_SESSION: SettingsUserDetails = SettingsUserDetails {
    next_break_duration_minutes: 120,
    active: true,
    allow_tracking: true,
    enable_on_startup: true,
};

#[specta::specta]
#[tauri::command]
pub fn load_settings(
    app_handle: AppHandle,
    settings: State<SettingsDetailsState>,
) -> model::settings::Settings {
    let version = app_handle.app_handle().config().version.clone();
    model::settings::Settings {
        app: model::settings::AppDetails {
            version: version.unwrap_or("unknown".to_string()),
        },
        user: settings.lock().unwrap().clone().unwrap_or(DEFAULT_SESSION),
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
            alert::alert(
                app_handle.app_handle(),
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
    let json_data = serde_json::to_value(settings_details)
        .map_err(|e| tauri_plugin_store::Error::Serialize(Box::new(e)))?;
    store.set("data".to_string(), json_data);
    store.save()?;
    Ok(())
}

fn load_settings_store(app: &AppHandle) -> Result<SettingsUserDetails, anyhow::Error> {
    let path = PathBuf::from(STORE_NAME);
    info!(
        "loading settings: {:?}",
        app.path().app_data_dir()?.join(&path)
    );
    let store = StoreBuilder::new(app.app_handle(), STORE_NAME).build();

    let data_json = store
        .get("data".to_string())
        .ok_or_else(|| tauri_plugin_store::Error::NotFound(PathBuf::from("data")))?;

    let details: SettingsUserDetails = serde_json::from_value(data_json.clone())
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
    load_settings_store(app_handle)
}

fn new<R>(app: &AppHandle<R>, start_with_about: bool) -> Result<WebviewWindow<R>, anyhow::Error>
where
    R: Runtime,
{
    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/settings".into()),
    )
        .title("Settings")
        .inner_size(800.0, 400.0)
        .center()
        .visible(false)
        .always_on_top(true)
        .transparent(true)
        .decorations(true)
        .skip_taskbar(false)
        .effects(WindowEffectsConfig::default())
        .resizable(false)
        .build()?;

    let handle = app.app_handle().clone();
    window.on_window_event(move |event| match event {
        WindowEvent::Focused(_) => {
            model::event::SettingsStartEvent { start_with_about }
                .emit(&handle)
                .unwrap();
        }
        _ => {}
    });
    Ok(window)
}

pub fn show<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    match app.get_webview_window(WINDOW_LABEL) {
        None => {
            new(app, false)?;
        }
        Some(w) => {
            if !w.is_visible()? {
                w.show()?;
            }
        }
    }
    Ok(())
}

pub fn show_about<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    match app.get_webview_window(WINDOW_LABEL) {
        None => {
            new(app, true)?;
        }
        Some(w) => {
            if !w.is_visible()? {
                w.show()?;
            }
        }
    }
    Ok(())
}
