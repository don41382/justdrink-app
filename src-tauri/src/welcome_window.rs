use crate::app_config::AppConfig;
use crate::model::device::DeviceId;
use crate::{tracking, TrackingState};
use log::warn;
use tauri::{AppHandle, Manager};

const WINDOW_LABEL: &str = "welcome";

pub fn show(app: &AppHandle, device_id: &DeviceId) -> Result<(), anyhow::Error> {
    let _window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/welcome".into()),
    )
    .title("Welcome to Drink Now!")
    .center()
    .transparent(true)
    .focused(true)
    .decorations(false)
    .resizable(false)
    .shadow(true)
    .skip_taskbar(false)
    .visible(false)
    .build()?;

    app.state::<TrackingState>()
        .send_tracking(tracking::Event::Install);
    open_thank_you(device_id);

    Ok(())
}

pub fn open_thank_you(device_id: &DeviceId) {
    if cfg!(feature = "fullversion") {
        // apple does not allow cross-reference
    } else {
        let url = format!(
            "{}/thank-you/{}?utm_source=app&utm_medium=install",
            AppConfig::build().get_url(),
            device_id.get_hash_hex_id()
        );
        match webbrowser::open(url.as_str()) {
            Ok(_) => {}
            Err(err) => {
                warn!("can't open thank you page with browser: {}", err);
            }
        }
    }
}
