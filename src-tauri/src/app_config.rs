use log::info;

pub struct AppConfig {
    url: String,
}

impl AppConfig {
    pub fn build() -> Self {
        if cfg!(feature = "local") {
            info!("local config");
            AppConfig {
                url: "http://localhost:8080".to_string()
            }
        } else {
            info!("remote config");
            AppConfig {
                url: "https://motionminute.app".to_string()
            }
        }
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}