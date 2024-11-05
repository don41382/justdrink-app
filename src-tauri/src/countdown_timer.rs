use crate::pretty_time::PrettyTime;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::cmp::{max, min};
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::AppHandle;
use tauri_specta::Event;
use timer::{Guard, Timer};

const TICKER_SPEED_MS: chrono::Duration = chrono::Duration::milliseconds(250);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event, PartialEq)]
pub struct CountdownEvent {
    pub(crate) status: TimerStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event, PartialEq)]
pub enum TimerStatus {
    NotStarted(u32),
    Active(u32),
    Paused(PauseOrigin, u32),
    Finished,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event, PartialEq)]
pub enum PauseOrigin {
    Idle,
    PreventSleep(String),
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event, PartialEq)]
pub enum ChangeTime {
    Add(u32),
    Remove(u32),
}

impl TimerStatus {
    pub fn is_running(&self) -> bool {
        match self {
            TimerStatus::Active(_) => true,
            _ => false,
        }
    }

    pub fn is_prevent_sleep(&self) -> bool {
        match self {
            TimerStatus::Paused(reason, _) => {
                matches!(reason, PauseOrigin::PreventSleep(_))
            }
            _ => false,
        }
    }

    pub fn to_text(&self) -> String {
        match self {
            TimerStatus::Active(duration) => Duration::from_secs(*duration as u64).to_pretty_time(),
            TimerStatus::Paused(origin, _) => match origin {
                PauseOrigin::Idle => "Paused due to idle".to_string(),
                PauseOrigin::PreventSleep(app_name) => format!("Paused by {}", app_name),
                PauseOrigin::User => "Next session is paused".to_string(),
            },
            TimerStatus::NotStarted(_) => "Not running".to_string(),
            TimerStatus::Finished => "Not running".to_string(),
        }
    }
}

pub struct CountdownTimer {
    timer: Timer,
    tick_callback: Arc<dyn Fn(TimerStatus) + Send + Sync>,
    duration: Arc<Mutex<Option<Duration>>>,
    remaining_time: Arc<Mutex<Duration>>,
    guard: Arc<Mutex<Option<Guard>>>,
    status: Arc<Mutex<TimerStatus>>,
}

impl CountdownTimer {
    pub fn new(app_handle: &AppHandle) -> Self {
        CountdownTimer {
            timer: Timer::new(),
            tick_callback: register_callback(app_handle),
            duration: Arc::new(Mutex::new(None)),
            remaining_time: Arc::new(Mutex::new(Duration::ZERO)),
            guard: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(TimerStatus::NotStarted(0))),
        }
    }

    /// Starts the countdown timer with the specified duration.
    pub fn start(&self, duration: Duration) {
        {
            let mut rem_time = self.remaining_time.lock().unwrap();
            *rem_time = duration;
        };

        {
            match self.duration.try_lock() {
                Ok(mut d) => {
                    *d = Some(duration);
                }
                Err(e) => {
                    println!("there is a lock on it {:?}", e);
                }
            }
        }

        let rem_time = Arc::clone(&self.remaining_time);
        let status = Arc::clone(&self.status);
        let guard_arc = Arc::clone(&self.guard);
        let callback = Arc::clone(&self.tick_callback);

        // Schedule the repeating task
        let guard = self.timer.schedule_repeating(TICKER_SPEED_MS, move || {
            // Check if paused
            {
                let status = { status.lock().unwrap().clone() };
                match status {
                    TimerStatus::Paused(_, _) => {
                        (*callback)(status.clone());
                        return;
                    }
                    _ => {}
                }
            }

            let rem_time: Duration = {
                let mut rem_time = rem_time.lock().unwrap();
                *rem_time = rem_time.saturating_sub(Duration::from_millis(
                    TICKER_SPEED_MS.num_milliseconds() as u64,
                ));
                rem_time.clone()
            };

            if rem_time > Duration::ZERO {
                let status = {
                    let mut status = status.lock().unwrap();
                    *status = TimerStatus::Active(rem_time.as_secs() as u32);
                    status.clone()
                };
                (*callback)(status);
            } else {
                {
                    // Stop the timer by dropping the guard
                    let mut guard_lock = guard_arc.lock().unwrap();
                    guard_lock.take(); // Dropping the guard cancels the timer
                }
                (*callback)(TimerStatus::Finished);
            }
        });

        // Store the guard to keep the scheduled task alive
        let mut guard_lock = self.guard.lock().unwrap();
        *guard_lock = Some(guard);
    }

    /// Pauses the countdown timer.
    pub fn pause(&self, pause_origin: PauseOrigin) {
        let mut paused = self.status.lock().unwrap();
        let remaining = self.remaining_time.lock().unwrap();
        *paused = TimerStatus::Paused(pause_origin, remaining.as_secs() as u32);
    }

    /// Resumes the countdown timer if it was paused.
    pub fn resume(&self) {
        let mut paused = self.status.lock().unwrap();
        let remaining_time = self.remaining_time.lock().unwrap();
        *paused = TimerStatus::Active(remaining_time.as_secs() as u32);
    }

    pub fn toggle(&self, pause_origin: PauseOrigin) {
        if matches!(self.timer_status(), TimerStatus::Paused(_, _)) {
            self.resume();
        } else {
            self.pause(pause_origin);
        }
    }

    pub fn change(&self, change_time: ChangeTime) {
        {
            let mut rem_time = self.remaining_time.lock().unwrap();
            match change_time {
                ChangeTime::Add(minutes) => {
                    *rem_time = rem_time.add(Duration::from_secs(minutes as u64 * 60))
                }
                ChangeTime::Remove(minutes) => {
                    let new_secs = rem_time.as_secs() as i32 - minutes as i32 * 60;
                    *rem_time = Duration::from_secs(max(new_secs, (minutes as i32) * 60) as u64);
                }
            }
        }
    }

    /// Stops the countdown timer.
    pub fn stop(&self) {
        // Cancel the scheduled task by dropping the guard
        {
            let mut guard_lock = self.guard.lock().unwrap();
            guard_lock.take();
        }

        // Reset the remaining time
        {
            let mut rem_time = self.remaining_time.lock().unwrap();
            *rem_time = Duration::ZERO;
        }
    }

    pub fn restart(&self) {
        let duration = {
            let locked_duration = *self.duration.lock().unwrap();
            locked_duration.clone()
        };
        if let Some(duration) = duration {
            self.start(duration);
        }
    }

    pub fn timer_status(&self) -> TimerStatus {
        self.status.lock().unwrap().clone()
    }
}

fn register_callback(app_handle: &AppHandle) -> Arc<dyn Fn(TimerStatus) + Send + Sync> {
    Arc::new({
        let app_handle_ticker = app_handle.clone();
        move |tick| {
            CountdownEvent {
                status: tick.clone(),
            }
            .emit(&app_handle_ticker)
            .unwrap();
        }
    })
}
