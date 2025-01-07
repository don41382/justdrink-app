use crate::tracking::Tracking;
use crate::{tracking, TrackingState};
use log::warn;
use tauri::{AppHandle, Manager};

const WINDOW_LABEL: &str = "welcome";

pub fn show(app: &AppHandle) -> Result<(), anyhow::Error> {
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
    open_thank_you();

    Ok(())
}

#[cfg(feature = "fullversion")]
pub fn open_thank_you() {}


#[cfg(not(feature = "fullversion"))]
pub fn open_thank_you() {
    let url = format!(
        "https://www.motionminute.app/thank-you/{}?utm_source=app&utm_medium=install",
        Tracking::get_machine_id()
    );
    match webbrowser::open(url.as_str()) {
        Ok(_) => {}
        Err(err) => {
            warn!("can't open thank you page with browser: {}", err);
        }
    }
}
