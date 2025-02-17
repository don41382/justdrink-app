pub struct AppConfig {
    url: String,
}

impl AppConfig {
    pub fn build() -> Self {
        if cfg!(feature = "local") {
            AppConfig {
                url: "http://drinknow.test:8080".to_string(),
            }
        } else {
            AppConfig {
                url: "https://drinknow.app".to_string(),
            }
        }
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}
