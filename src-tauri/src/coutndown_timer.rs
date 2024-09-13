use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct CountdownTimer {
    tick_callback: Arc<Mutex<Option<Box<dyn Fn(Duration) + Send + 'static>>>>,
    finish_callback: Arc<Mutex<Option<Box<dyn Fn() + Send + 'static>>>>,
    is_paused: Arc<AtomicBool>,
    is_stopped: Arc<AtomicBool>,
    current_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}

impl CountdownTimer {
    pub fn new() -> Self {
        CountdownTimer {
            tick_callback: Arc::new(Mutex::new(None)),
            finish_callback: Arc::new(Mutex::new(None)),
            is_paused: Arc::new(AtomicBool::new(false)),
            is_stopped: Arc::new(AtomicBool::new(false)),
            current_thread: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_tick_callback(&self, callback: Box<dyn Fn(Duration) + Send + 'static>) {
        *self.tick_callback.lock().unwrap() = Some(callback);
    }

    pub fn set_finish_callback(&self, callback: Box<dyn Fn() + Send + 'static>) {
        *self.finish_callback.lock().unwrap() = Some(callback);
    }

    pub fn start(&self, duration: Duration) {
        self.stop(); // Stop any running timer before starting a new one

        let tick_callback = Arc::clone(&self.tick_callback);
        let finish_callback = Arc::clone(&self.finish_callback);
        let is_paused = Arc::clone(&self.is_paused);
        let is_stopped = Arc::clone(&self.is_stopped);
        let current_thread = Arc::clone(&self.current_thread);

        is_paused.store(false, Ordering::SeqCst);
        is_stopped.store(false, Ordering::SeqCst);

        let handle = thread::spawn(move || {
            let start_time = Instant::now();
            let mut elapsed = Duration::new(0, 0);

            while elapsed < duration && !is_stopped.load(Ordering::SeqCst) {
                if is_paused.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                elapsed = Instant::now() - start_time;

                if let Some(ref callback) = *tick_callback.lock().unwrap() {
                    callback(duration - elapsed);
                }

                thread::sleep(Duration::from_millis(1000));
            }

            if !is_stopped.load(Ordering::SeqCst) {
                if let Some(ref callback) = *finish_callback.lock().unwrap() {
                    callback();
                }
            }
        });

        *current_thread.lock().unwrap() = Some(handle);
    }

    pub fn pause(&self) {
        self.is_paused.store(true, Ordering::SeqCst);
    }

    pub fn resume(&self) {
        self.is_paused.store(false, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.is_stopped.store(true, Ordering::SeqCst);

        if let Some(handle) = self.current_thread.lock().unwrap().take() {
            handle.join().unwrap();
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

        assert_eq!(received_ticks, vec![2, 1, 0]);
    }

    #[test]
    fn test_pause_resume() {
        let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();

        let timer = CountdownTimer::new();
        timer.set_tick_callback(Box::new(move |remaining_time| {
            tx.send(remaining_time.as_secs()).unwrap();
        }));

        timer.start(Duration::from_secs(5));
        sleep(Duration::from_secs(2));
        timer.pause();
        sleep(Duration::from_secs(2));
        timer.resume();
        sleep(Duration::from_secs(4));

        let mut received_ticks = vec![];
        while let Ok(tick) = rx.try_recv() {
            received_ticks.push(tick);
        }

        assert_eq!(received_ticks, vec![4, 3, 2, 1, 0]);
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

        assert_eq!(received_ticks, vec![4, 3]);
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

        assert_eq!(received_ticks, vec![2, 1, 0]);
    }
}