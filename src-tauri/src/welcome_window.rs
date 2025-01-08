use crate::{tracking, TrackingState};
use log::warn;
use tauri::{AppHandle, Manager};
use crate::model::device::DeviceId;

const WINDOW_LABEL: &str = "welcome";

pub fn show(app: &AppHandle, device_id: &DeviceId) -> Result<(), anyhow::Error> {
    let _window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/welcome".into()),
    )
    .title("Welcome to Motion Minute")
    .center()
    .transparent(true)
    .always_on_top(true)
    .focused(true)
    .decorations(false)
    .resizable(false)
    .shadow(true)
    .visible(false)
    .build()?;

    app.state::<TrackingState>()
        .send_tracking(tracking::Event::Install);
    open_thank_you(device_id);

    Ok(())
}

#[cfg(feature = "fullversion")]
pub fn open_thank_you(device_id: &DeviceId) {}


#[cfg(not(feature = "fullversion"))]
pub fn open_thank_you(device_id: &DeviceId) {
    let url = format!(
        "https://www.motionminute.app/thank-you/{}?utm_source=app&utm_medium=install",
        device_id.get_hash_hex_id()
    );
    match webbrowser::open(url.as_str()) {
        Ok(_) => {}
        Err(err) => {
            warn!("can't open thank you page with browser: {}", err);
        }
    }
}
