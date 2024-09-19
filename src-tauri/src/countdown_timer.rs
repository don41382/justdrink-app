use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::{Arc, Mutex, TryLockResult};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_specta::Event;
use timer::{Guard, Timer};

pub const EVENT_FINISHED: &'static str = "event_finished";

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct EventTicker {
    pub countdown: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event, PartialEq)]
pub enum TickerStatus {
    START,
    FINISHED,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct EventTickerStatus {
    pub status: TickerStatus,
}


pub struct CountdownTimer {
    timer: Timer,
    tick_callback: Arc<Mutex<Option<Box<dyn Fn(Duration) + Send + 'static>>>>,
    status_callback: Arc<Mutex<Option<Box<dyn Fn(TickerStatus) + Send + 'static>>>>,
    duration: Arc<Mutex<Option<Duration>>>,
    remaining_time: Arc<Mutex<Duration>>,
    guard: Arc<Mutex<Option<Guard>>>,
    is_paused: Arc<Mutex<bool>>,
}

impl CountdownTimer {
    pub fn new() -> Self {
        CountdownTimer {
            timer: Timer::new(),
            tick_callback: Arc::new(Mutex::new(None)),
            status_callback: Arc::new(Mutex::new(None)),
            duration: Arc::new(Mutex::new(None)),
            remaining_time: Arc::new(Mutex::new(Duration::ZERO)),
            guard: Arc::new(Mutex::new(None)),
            is_paused: Arc::new(Mutex::new(false)),
        }
    }


    fn set_tick_callback(&self, callback: Box<dyn Fn(Duration) + Send + 'static>) {
        let mut tick_cb = self.tick_callback.lock().unwrap();
        *tick_cb = Some(callback);
    }

    fn set_status_callback(&self, callback: Box<dyn Fn(TickerStatus) + Send + 'static>) {
        let mut finish_cb = self.status_callback.lock().unwrap();
        *finish_cb = Some(callback);
    }

    pub fn register_callback(&self, app_handle: AppHandle) {
        let mut cb_finished = self.status_callback.lock().unwrap();
        let mut cb_tick = self.tick_callback.lock().unwrap();

        *cb_finished = Some(Box::new({
            let app_handle_finish = app_handle.clone();
            move |status| {
                EventTickerStatus { status }.emit(&app_handle_finish).unwrap();
            }
        }));

        *cb_tick = Some(Box::new({
            let app_handle_ticker = app_handle.clone();
            move |tick| {
                EventTicker { countdown: tick.as_secs() as u32 }.emit(&app_handle_ticker).unwrap();
            }
        }));
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
                    println!("works");
                    *d = Some(duration);
                }
                Err(e) => {
                    println!("there is a lock on it {:?}", e);
                }
            }
        }

        let tick_cb = Arc::clone(&self.tick_callback);
        let finish_cb = Arc::clone(&self.status_callback);
        let rem_time = Arc::clone(&self.remaining_time);
        let is_paused = Arc::clone(&self.is_paused);
        let guard_arc = Arc::clone(&self.guard);

        // Schedule the repeating task
        let guard = self
            .timer
            .schedule_repeating(chrono::Duration::seconds(1), move || {
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
                    *rem_time = *rem_time - Duration::from_secs(1);
                    rem_time.clone()
                };

                if let Some(ref callback) = *tick_cb.lock().unwrap() {
                    callback(rem_time);
                }

                if rem_time <= Duration::ZERO {
                    {
                        // Stop the timer by dropping the guard
                        let mut guard_lock = guard_arc.lock().unwrap();
                        guard_lock.take(); // Dropping the guard cancels the timer
                    }


                    // Time's up
                    if let Some(ref callback) = *finish_cb.lock().unwrap() {
                        callback(TickerStatus::FINISHED);
                    }
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

        // send zero callback
        {
            if let Some(ref callback) = *self.tick_callback.lock().unwrap() {
                callback(Duration::ZERO);
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::thread::sleep;

    #[test]
    fn test_elapse() {
        let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();

        let timer = CountdownTimer::new();
        timer.set_tick_callback(Box::new(move |remaining_time| {
            tx.send(remaining_time.as_secs()).unwrap();
        }));

        timer.start(Duration::from_secs(3));
        sleep(Duration::from_secs(4));

        let mut received_ticks = vec![];
        while let Ok(tick) = rx.try_recv() {
            received_ticks.push(tick);
        }

        assert_eq!(received_ticks, vec![2, 1]);
    }

    #[test]
    fn test_pause_resume() {
        let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();

        println!("new");
        let timer = CountdownTimer::new();
        timer.set_tick_callback(Box::new(move |remaining_time| {
            tx.send(remaining_time.as_secs()).unwrap();
        }));

        println!("start");
        timer.start(Duration::from_secs(5));
        sleep(Duration::from_secs(2));
        println!("pause");
        timer.pause();
        sleep(Duration::from_secs(2));
        println!("resume");
        timer.resume();
        sleep(Duration::from_secs(4));

        let mut received_ticks = vec![];
        while let Ok(tick) = rx.try_recv() {
            received_ticks.push(tick);
        }

        assert_eq!(received_ticks, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_stop() {
        let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();

        let timer = CountdownTimer::new();
        timer.set_tick_callback(Box::new(move |remaining_time| {
            tx.send(remaining_time.as_secs()).unwrap();
        }));

        timer.start(Duration::from_secs(5));
        sleep(Duration::from_secs(2));
        timer.stop();
        sleep(Duration::from_secs(2));

        let mut received_ticks = vec![];
        while let Ok(tick) = rx.try_recv() {
            received_ticks.push(tick);
        }

        assert_eq!(received_ticks, vec![4]);
    }

    #[test]
    fn test_delayed_callback_registration() {
        let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();

        let timer = CountdownTimer::new();
        timer.start(Duration::from_secs(5));

        sleep(Duration::from_secs(2));

        timer.set_tick_callback(Box::new(move |remaining_time| {
            tx.send(remaining_time.as_secs()).unwrap();
        }));

        sleep(Duration::from_secs(4));

        let mut received_ticks = vec![];
        while let Ok(tick) = rx.try_recv() {
            received_ticks.push(tick);
        }

        assert_eq!(received_ticks, vec![2, 1]);
    }
}
