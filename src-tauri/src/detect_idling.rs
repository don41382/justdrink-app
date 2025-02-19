use crate::countdown_timer::{PauseOrigin, TimerStatus};
use crate::{CountdownTimerState, SettingsManagerState};
use log::{debug};
use std::thread::sleep;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use user_idle::UserIdle;

const IDLE_DURATION: u64 = 60;

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
            let settings = app_handle.app_handle().state::<SettingsManagerState>();
            let idle = UserIdle::get_time().unwrap();

            if let Some(settings) = settings.get_settings() {
                let _timer_duration = if let TimerStatus::Active(duration) = timer.timer_status() {
                    Some(duration)
                } else {
                    None
                };

                if settings.user.active && settings.user.enable_idle_detection {
                    match mode {
                        Mode::Pause => {
                            if idle.as_seconds() < IDLE_DURATION {
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
                            if idle.as_seconds() > IDLE_DURATION {
                                debug!("switch to pause");
                                timer.pause(PauseOrigin::Idle);
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
