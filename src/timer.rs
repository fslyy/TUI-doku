use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct GameTimer {
    start_time: Option<Instant>,
    elapsed: Duration,
    paused: bool,
}

impl GameTimer {
    pub fn new() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::from_secs(0),
            paused: false,
        }
    }

    pub fn start(&mut self, elapsed: Option<u64>) {
        self.start_time = Some(Instant::now());
        self.elapsed = Duration::from_secs(elapsed.unwrap_or(0));
        self.paused = false;
    }

    pub fn pause(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed += start.elapsed();
            self.start_time = None;
            self.paused = true;
        }
    }

    pub fn resume(&mut self) {
        if self.paused {
            self.start_time = Some(Instant::now());
            self.paused = false;
        }
    }

    pub fn elapsed(&self) -> Duration {
        if let Some(start) = self.start_time {
            self.elapsed + start.elapsed()
        } else {
            self.elapsed
        }
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::from_secs(0);
        self.paused = false;
    }
}
