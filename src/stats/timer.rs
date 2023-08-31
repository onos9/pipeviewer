use std::time::{Duration, Instant};

pub struct Timer {
    pub last_instance: Instant,
    pub delta: Duration,
    pub period: Duration,
    pub countdown: Duration,
    pub ready: bool,
}

impl Default for Timer {
    fn default() -> Timer {
        Timer::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instance: now,
            delta: Duration::default(),
            period: Duration::from_millis(100),
            countdown: Duration::default(),
            ready: true,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instance;
        self.last_instance = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
