use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use anyhow::{Result, anyhow};
use log::{debug, warn};
use tauri::{AppHandle, Manager};
use user_idle::UserIdle;
use crate::{CountdownTimerState, SettingsDetailsState};
use crate::countdown_timer::{PauseOrigin, TimerStatus};

#[cfg(target_os = "macos")]
fn sleep_is_prevented_by_apps() -> Result<bool, anyhow::Error> {
    let output = Command::new("pmset")
        .arg("-g")
        .output()
        .map_err(|e| anyhow!("Failed to execute pmset: {}", e))?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    if output_str.contains("display sleep prevented by") {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(target_os = "windows")]
fn sleep_is_prevented_by_apps() -> Result<bool, anyhow::Error> {
    // Placeholder implementation for Windows.
    // On Windows, detecting if audio is playing can be done through the Windows API (e.g., using the Core Audio APIs).
    // This is a non-trivial task and may require using crates like `winapi` or `cpal`.

    // For now, we'll return an Ok(false) to indicate audio is not being detected.
    Ok(false)
}

pub enum Mode {
    Pause,
    Working,
}

pub fn init<R>(app: &AppHandle<R>) -> Result<(), anyhow::Error>
where
    R: tauri::Runtime,
{
    let app_handle = app.app_handle().clone();
    tauri::async_runtime::spawn(async move {
        let timer = app_handle.state::<CountdownTimerState>();
        let mut mode = Mode::Working;
        loop {
            let settings = app_handle.app_handle().state::<SettingsDetailsState>();
            let idle = UserIdle::get_time().unwrap();

            if let Ok(s) = settings.try_lock() {
                let sleep_is_prevented = sleep_is_prevented_by_apps().unwrap_or_else(|e| {
                    warn!("can't run audio check: ${:?}", e);
                    false
                });

                if s.as_ref().map(|s| s.active).unwrap_or(false) {
                    match mode {
                        Mode::Pause => {
                            if !sleep_is_prevented && idle.as_seconds() < 60 {
                                debug!("switch to working");
                                if timer.timer_status() == TimerStatus::Paused(PauseOrigin::IdleOrVideo) {
                                    timer.resume();
                                }
                                mode = Mode::Working;
                            }
                        }
                        Mode::Working => {
                            if sleep_is_prevented || idle.as_seconds() > 60 {
                                debug!("switch to pause");
                                timer.pause(PauseOrigin::IdleOrVideo);
                                mode = Mode::Pause;
                            }
                        }
                    }
                }
            }
            sleep(Duration::from_secs(1));
        }
    });

    Ok(())
}