use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) struct SessionTimer {
    duration: Duration,
    start_time: Option<Instant>,
    is_running: Arc<Mutex<bool>>,
    tick_callback: Option<Arc<Mutex<Box<dyn Fn(Duration) + Send + 'static>>>>,
    end_callback: Option<Arc<Mutex<Box<dyn Fn() + Send + 'static>>>>,
}

impl SessionTimer {
    pub fn new() -> Self {
        SessionTimer {
            duration: Duration::from_secs(0),
            start_time: None,
            is_running: Arc::new(Mutex::new(false)),
            tick_callback: None,
            end_callback: None,
        }
    }

    pub fn register<F, G>(&mut self, tick_callback: F, end_callback: G)
    where
        F: Fn(Duration) + Send + 'static,
        G: Fn() + Send + 'static,
    {
        self.tick_callback = Some(Arc::new(Mutex::new(Box::new(tick_callback))));
        self.end_callback = Some(Arc::new(Mutex::new(Box::new(end_callback))));
    }

    pub fn start(&mut self, duration: Duration) {
        let mut is_running = self.is_running.lock().unwrap();
        self.duration = duration;
        self.start_time = Some(Instant::now());

        if !*is_running {
            *is_running = true;

            let is_running = Arc::clone(&self.is_running);
            let tick_callback = self.tick_callback.as_ref().map(Arc::clone);
            let end_callback = self.end_callback.as_ref().map(Arc::clone);

            thread::spawn(move || {
                let start = Instant::now();
                while start.elapsed() < duration {
                    if !*is_running.lock().unwrap() {
                        return;
                    }
                    let remaining = duration - start.elapsed();
                    if let Some(cb) = &tick_callback {
                        cb.lock().unwrap()(remaining);
                    }
                    thread::sleep(Duration::from_secs(1));
                }
                if let Some(cb) = &end_callback {
                    cb.lock().unwrap()();
                }
                *is_running.lock().unwrap() = false;
            });
        } else {
            // Reset the timer if it's already running
            self.start_time = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        self.start_time = None;
    }

    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        self.start_time.map(|start| {
            let elapsed = start.elapsed();
            if elapsed >= self.duration {
                Duration::from_secs(0)
            } else {
                self.duration - elapsed
            }
        })
    }
}