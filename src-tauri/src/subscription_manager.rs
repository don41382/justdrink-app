use crate::app_config::AppConfig;
use crate::model::device::DeviceId;
use serde::Serialize;
use tauri_plugin_http::reqwest::blocking::Client;

pub struct SubscriptionManager {
    client: Client,
    device_id: DeviceId,
}

#[derive(Serialize, Clone)]
struct SubscribeRequest {
    email: Option<String>,
    did: String,
    subscribe: bool,
}

impl SubscriptionManager {
    pub fn new(device_id: DeviceId) -> Self {
        SubscriptionManager {
            client: Client::new(),
            device_id,
        }
    }

    pub fn subscribe(&self, email: Option<String>, subscribe: bool) -> Result<(), anyhow::Error> {
        let request = SubscribeRequest {
            subscribe,
            did: self.device_id.get_hash_hex_id(),
            email,
        };

        let response = self
            .client
            .post(format!(
                "{}/app/v1/newsletter/subscribe",
                AppConfig::build().get_url()
            ))
            .form(&request)
            .send()?;

        response.error_for_status()?;
        Ok(())
    }
}
