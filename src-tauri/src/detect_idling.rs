use crate::countdown_timer::{PauseOrigin, TimerStatus};
use crate::idle;
use crate::{CountdownTimerState, SettingsDetailsState};
use log::{debug, warn};
use std::thread::sleep;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use user_idle::UserIdle;

const SLEEP_PREVENTION_PAUSE_THRESHOLD_SECS: u32 = 120;

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
                let sleep_prevented_by = idle::sleep_is_prevented_by_apps().unwrap_or_else(|e| {
                    warn!("can't run audio check: ${:?}", e);
                    None
                });

                let timer_duration = if let TimerStatus::Active(duration) = timer.timer_status() {
                    Some(duration)
                } else {
                    None
                };

                if s.as_ref().map(|s| s.active && s.enable_idle_detection).unwrap_or(false) {
                    match mode {
                        Mode::Pause => {
                            if sleep_prevented_by.is_none() && idle.as_seconds() < 60 {
                                debug!("switch to working");
                                if matches!(
                                    timer.timer_status(),
                                    TimerStatus::Paused(PauseOrigin::Idle, _)
                                ) || timer.timer_status().is_prevent_sleep()
                                {
                                    timer.resume();
                                }
                                mode = Mode::Working;
                            }
                        }
                        Mode::Working => {
                            if idle.as_seconds() > 60 {
                                debug!("switch to pause");
                                timer.pause(PauseOrigin::Idle);
                                mode = Mode::Pause;
                            }
                            // Pause the timer only if sleep is prevented and timer duration < X minutes
                            else if let Some(app_name) = sleep_prevented_by {
                                if timer_duration
                                    .map_or(false, |d| d < SLEEP_PREVENTION_PAUSE_THRESHOLD_SECS)
                                {
                                    debug!("switch to pause due to sleep prevention and timer duration {} seconds", SLEEP_PREVENTION_PAUSE_THRESHOLD_SECS);
                                    timer.pause(PauseOrigin::PreventSleep(app_name));
                                    mode = Mode::Pause;
                                }
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
