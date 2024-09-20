use crate::countdown_timer::CountdownStatus::{Finished, RunningSeconds};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::AppHandle;
use tauri_specta::Event;
use timer::{Guard, Timer};

const TICKER_SPEED_MS: chrono::Duration = chrono::Duration::milliseconds(500);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event, PartialEq)]
pub struct CountdownEvent {
    pub(crate) status: CountdownStatus,
}


#[derive(Serialize, Deserialize, Debug, Clone, Type, Event, PartialEq)]
pub enum CountdownStatus {
    Start,
    RunningSeconds { countdown_seconds: u32 },
    Finished,
}

pub struct CountdownTimer {
    timer: Timer,
    tick_callback: Arc<dyn Fn(CountdownStatus) + Send + Sync>,
    duration: Arc<Mutex<Option<Duration>>>,
    remaining_time: Arc<Mutex<Duration>>,
    guard: Arc<Mutex<Option<Guard>>>,
    is_paused: Arc<Mutex<bool>>,
}

impl CountdownTimer {
    pub fn new(app_handle: &AppHandle) -> Self {
        CountdownTimer {
            timer: Timer::new(),
            tick_callback: register_callback(app_handle),
            duration: Arc::new(Mutex::new(None)),
            remaining_time: Arc::new(Mutex::new(Duration::ZERO)),
            guard: Arc::new(Mutex::new(None)),
            is_paused: Arc::new(Mutex::new(false)),
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
        let is_paused = Arc::clone(&self.is_paused);
        let guard_arc = Arc::clone(&self.guard);
        let callback = Arc::clone(&self.tick_callback);

        // Schedule the repeating task
        let guard = self.timer.schedule_repeating(TICKER_SPEED_MS, move || {
            // Check if paused
            {
                let paused = is_paused.lock().unwrap();
                if *paused {
                    return;
                }
                drop(paused); // Release the lock before proceeding
            }

            let rem_time: Duration = {
                let mut rem_time = rem_time.lock().unwrap();
                *rem_time =
                    *rem_time - Duration::from_millis(TICKER_SPEED_MS.num_milliseconds() as u64);
                rem_time.clone()
            };

            (*callback)(RunningSeconds { countdown_seconds: rem_time.as_secs() as u32 });

            if rem_time <= Duration::ZERO {
                {
                    // Stop the timer by dropping the guard
                    let mut guard_lock = guard_arc.lock().unwrap();
                    guard_lock.take(); // Dropping the guard cancels the timer
                }

                (*callback)(Finished);
            }
        });

        // Store the guard to keep the scheduled task alive
        let mut guard_lock = self.guard.lock().unwrap();
        *guard_lock = Some(guard);
    }

    /// Pauses the countdown timer.
    #[allow(dead_code)]
    pub fn pause(&self) {
        let mut paused = self.is_paused.lock().unwrap();
        *paused = true;
    }

    /// Resumes the countdown timer if it was paused.
    #[allow(dead_code)]
    pub fn resume(&self) {
        let mut paused = self.is_paused.lock().unwrap();
        *paused = false;
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
}

fn register_callback(app_handle: &AppHandle) -> Arc<dyn Fn(CountdownStatus) + Send + Sync> {
    Arc::new({
        let app_handle_ticker = app_handle.clone();
        move |tick| {
            CountdownEvent {
                status: tick.clone()
            }.emit(&app_handle_ticker).unwrap();
        }
    })
}
