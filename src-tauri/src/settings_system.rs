use crate::model::settings::SettingsSystemDetails;
use anyhow::Error;
use chrono::{Duration, Utc};
use log::{debug, error, warn};
use std::string::ToString;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::StoreBuilder;

const STORE_NAME: &str = "mm-system-config.json";
const ROOT_PATH: &str = "system";

#[derive(Clone)]
pub(crate) struct SettingsSystem {
    settings: SettingsSystemDetails,
}

impl SettingsSystem {
    pub fn load(app: &AppHandle) -> SettingsSystem {
        let settings = Self::load_settings_store(app).unwrap_or_else(|err| {
            warn!("system store settings not found: {:?}", err);
            SettingsSystemDetails {
                session_count: 0,
                feedback_provided: false,
                last_update_check_date: Utc::now(),
            }
        });
        SettingsSystem { settings }
    }

    pub fn settings(&self) -> SettingsSystemDetails {
        self.settings.clone()
    }

    pub fn feedback_given<R>(&mut self, app: &AppHandle<R>) -> ()
    where
        R: Runtime,
    {
        self.settings.feedback_provided = true;
        self.write_settings(app).unwrap_or_else(|err| {
            error!("unable to write system settings {}", err)
        })
    }

    pub fn increase_session_count<R>(&mut self, app: &AppHandle<R>) -> ()
    where
        R: Runtime,
    {
        self.settings.session_count += 1;
        self.write_settings(app).unwrap_or_else(|err| {
            error!("unable to write system settings {}", err)
        })
    }

    pub fn set_last_check_date<R>(&mut self, app: &AppHandle<R>) -> Result<(), Error>
    where
        R: Runtime,
    {
        self.settings.last_update_check_date = Utc::now();
        self.write_settings(app)?;
        Ok(())
    }

    pub fn updater_check_needed(&self) -> bool {
        (self.settings.last_update_check_date + Duration::days(2)) < Utc::now()
    }

    fn write_settings<R>(&self, app: &AppHandle<R>) -> Result<(), anyhow::Error>
    where
        R: Runtime,
    {
        debug!("save system settings");
        let store = StoreBuilder::new(app.app_handle(), STORE_NAME).build()?;

        let json_data = serde_json::to_value(self.settings.clone())
            .map_err(|e| tauri_plugin_store::Error::Serialize(Box::new(e)))?;

        store.set(ROOT_PATH.to_string(), json_data);
        store.save()?;

        Ok(())
    }

    fn load_settings_store(app: &AppHandle) -> Result<SettingsSystemDetails, anyhow::Error> {
        let store = StoreBuilder::new(app.app_handle(), STORE_NAME).build()?;

        let data_json = store
            .get(ROOT_PATH.to_string())
            .ok_or_else(|| anyhow::anyhow!("can't find settings in {}", ROOT_PATH))?;

        let settings: SettingsSystemDetails = serde_json::from_value(data_json.clone())
            .map_err(|e| tauri_plugin_store::Error::Deserialize(Box::new(e)))?;

        Ok(settings)
    }
}
