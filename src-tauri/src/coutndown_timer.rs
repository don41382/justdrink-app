use std::sync::{Arc, Mutex};
use std::time::Duration;
use timer::{Guard, Timer};

pub struct CountdownTimer {
    timer: Timer,
    tick_callback: Arc<Mutex<Option<Box<dyn Fn(Duration) + Send + 'static>>>>,
    finish_callback: Arc<Mutex<Option<Box<dyn Fn() + Send + 'static>>>>,
    remaining_time: Arc<Mutex<Duration>>,
    guard: Arc<Mutex<Option<Guard>>>,
    is_paused: Arc<Mutex<bool>>,
}

impl CountdownTimer {
    /// Creates a new instance of `CountdownTimer`.
    pub fn new() -> Self {
        CountdownTimer {
            timer: Timer::new(),
            tick_callback: Arc::new(Mutex::new(None)),
            finish_callback: Arc::new(Mutex::new(None)),
            remaining_time: Arc::new(Mutex::new(Duration::ZERO)),
            guard: Arc::new(Mutex::new(None)),
            is_paused: Arc::new(Mutex::new(false)),
        }
    }

    /// Sets the tick callback, which is called every second with the remaining time.
    pub fn set_tick_callback(&self, callback: Box<dyn Fn(Duration) + Send + 'static>) {
        let mut tick_cb = self.tick_callback.lock().unwrap();
        *tick_cb = Some(callback);
    }

    /// Sets the finish callback, which is called when the countdown reaches zero.
    pub fn set_finish_callback(&self, callback: Box<dyn Fn() + Send + 'static>) {
        let mut finish_cb = self.finish_callback.lock().unwrap();
        *finish_cb = Some(callback);
    }

    /// Starts the countdown timer with the specified duration.
    pub fn start(&self, duration: Duration) {
        {
            let mut rem_time = self.remaining_time.lock().unwrap();
            *rem_time = duration;
        }
        let tick_cb = Arc::clone(&self.tick_callback);
        let finish_cb = Arc::clone(&self.finish_callback);
        let rem_time = Arc::clone(&self.remaining_time);
        let is_paused = Arc::clone(&self.is_paused);
        let guard_arc = Arc::clone(&self.guard);

        // Schedule the repeating task
        let guard = self.timer.schedule_repeating(chrono::Duration::seconds(1), move || {
            // Check if paused
            let paused = is_paused.lock().unwrap();
            if *paused {
                return;
            }
            drop(paused); // Release the lock before proceeding

            // Decrement the remaining time
            let mut rem_time = rem_time.lock().unwrap();
            *rem_time = *rem_time - Duration::from_secs(1);

            if *rem_time <= Duration::ZERO {
                // Time's up
                if let Some(ref callback) = *finish_cb.lock().unwrap() {
                    callback();
                }

                // Stop the timer by dropping the guard
                let mut guard_lock = guard_arc.lock().unwrap();
                guard_lock.take(); // Dropping the guard cancels the timer
            } else {
                // Call the tick callback with the remaining time
                if let Some(ref callback) = *tick_cb.lock().unwrap() {
                    callback(*rem_time);
                }
            }
        });

        // Store the guard to keep the scheduled task alive
        let mut guard_lock = self.guard.lock().unwrap();
        *guard_lock = Some(guard);
    }

    /// Pauses the countdown timer.
    pub fn pause(&self) {
        let mut paused = self.is_paused.lock().unwrap();
        *paused = true;
    }

    /// Resumes the countdown timer if it was paused.
    pub fn resume(&self) {
        let mut paused = self.is_paused.lock().unwrap();
        *paused = false;
    }

    /// Stops the countdown timer.
    pub fn stop(&self) {
        // Cancel the scheduled task by dropping the guard
        let mut guard_lock = self.guard.lock().unwrap();
        guard_lock.take();

        // Reset the remaining time
        let mut rem_time = self.remaining_time.lock().unwrap();
        *rem_time = Duration::ZERO;

        // send zero callback
        if let Some(ref callback) = *self.tick_callback.lock().unwrap() {
            callback(Duration::ZERO);
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