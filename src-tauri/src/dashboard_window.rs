use crate::countdown_timer::{PauseOrigin, TimerStatus};
use crate::{countdown_timer, CountdownTimerState};
use std::env;
use tauri::{AppHandle, Manager, Runtime};

pub(crate) const WINDOW_LABEL: &'static str = "dashboard";

pub fn show<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: Runtime,
{
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        window.show()?;
        window.set_focus()?;
    } else {
        let _window = tauri::WebviewWindowBuilder::new(
            app,
            WINDOW_LABEL,
            tauri::WebviewUrl::App("/dashboard".into()),
        )
        .title("Drink Now! - Dashboard")
        .center()
            .focused(true)
        .transparent(true)
        .decorations(false)
        .shadow(true)
        .resizable(false)
        .visible(false)
        .build()?;
    }

    Ok(())
}

pub fn should_show_dashboard() -> bool {
    let args: Vec<String> = env::args().collect();
    !args.contains(&"--quiet".to_string())
}

#[specta::specta]
#[tauri::command]
pub async fn toggle_timer(app: AppHandle) {
    app.state::<CountdownTimerState>().toggle(PauseOrigin::User);
}

#[specta::specta]
#[tauri::command]
pub async fn timer_change(app: AppHandle, change_time: countdown_timer::ChangeTime) {
    app.state::<CountdownTimerState>().change(change_time);
}

#[specta::specta]
#[tauri::command]
pub fn get_current_timer_status(app: AppHandle) -> TimerStatus {
    app.state::<CountdownTimerState>().timer_status()
}
