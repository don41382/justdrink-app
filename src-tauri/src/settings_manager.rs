use crate::model::session::{DrinkCharacter, SipSize};
use crate::model::settings::{
    AppDetails, Settings, SettingsTabs, SettingsUserDetails
};
use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager}; // For Tauri state management
use tauri_plugin_store::{Store, StoreBuilder};

const STORE_NAME: &str = "mm-config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettingsStore {
    pub version: String,
    pub user: SettingsUserDetails,
}

impl Default for UserSettingsStore {
    fn default() -> Self {
        UserSettingsStore {
            version: "0.0.0".to_string(),
            user: SettingsUserDetails {
                character: DrinkCharacter::YoungMan,
                sip_size: SipSize::FullCup,
                consent: false,
                next_break_duration_minutes: 15,
                drink_amount_ml: 3000,
                active: true,
                beta_version: false,
                enable_on_startup: true,
                enable_idle_detection: true,
                allow_tracking: true,
            },
        }
    }
}

pub struct SettingsManager {
    store: Arc<Store<tauri::Wry>>,
    version: String,
    settings: Mutex<Option<UserSettingsStore>>,
}

impl SettingsManager {
    // Initialize the SettingsManager
    pub fn new(app: &AppHandle) -> Result<Self> {
        let store = StoreBuilder::new(app.app_handle(), STORE_NAME).build()?;
        let settings = Mutex::new(None);
        let version = app
            .config()
            .version
            .clone()
            .unwrap_or_else(|| "0.0.0".to_string());
        let sm = Self {
            store,
            version,
            settings,
        };
        sm.load()
            .unwrap_or_else(|err| warn!("unable to read configuration, err: {:?}", err));
        Ok(sm)
    }

    // Load settings from the store
    pub fn load(&self) -> Result<()> {
        let data_json = self
            .store
            .get("data".to_string())
            .ok_or_else(|| anyhow::anyhow!("Can't find settings in data"))?;

        let user_settings: UserSettingsStore = serde_json::from_value(data_json.clone())?;

        {
            let mut settings_guard = self.settings.lock().map_err(|e| {
                anyhow::anyhow!("Failed to lock settings - mutex poisoned: {:?}", e)
            })?;
            *settings_guard = Some(user_settings.clone());
        }
        Ok(())
    }

    pub fn update_user(&self, user_settings: SettingsUserDetails) -> Result<()> {
        {
            let mut settings_guard = self.settings.lock().map_err(|e| {
                anyhow::anyhow!("Failed to lock settings - mutex poisoned: {:?}", e)
            })?;

            match *settings_guard {
                Some(ref mut current_settings) => {
                    current_settings.user = user_settings.clone();
                }
                None => {
                    *settings_guard = Some(UserSettingsStore {
                        version: self.version.clone(),
                        user: user_settings.clone(),
                    });
                }
            }
        }
        self.save()?;
        Ok(())
    }

    // Save settings to the store
    pub fn save(&self) -> Result<()> {
        info!("Saving settings...");

        let settings = {
            let settings = self.settings.lock().map_err(|e| {
                anyhow::anyhow!("Failed to lock settings - mutex poisoned: {:?}", e)
            })?;
            settings.clone()
        };

        if let Some(s) = settings {
            // Serialize the settings
            let json_data = serde_json::to_value(s)?;

            // Update the store
            self.store.set("data".to_string(), json_data);
            self.store.save()?;
        }

        Ok(())
    }

    pub fn get_settings(&self) -> Option<UserSettingsStore> {
        let settings = self
            .settings
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock settings - mutex poisoned: {:?}", e))
            .expect("get settings - should not be poisoned");
        settings.clone()
    }
}
