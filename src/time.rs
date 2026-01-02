use std::fmt;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Clock {
    frame_duration: Duration,
    last_tick: Instant,
}

impl Clock {
    pub fn from_fps(fps: u32) -> Result<Self, ClockError> {
        if fps == 0 {
            return Err(ClockError::InvalidFps { fps });
        }
        let nanos_per_frame = 1_000_000_000u64 / u64::from(fps);
        Ok(Self {
            frame_duration: Duration::from_nanos(nanos_per_frame),
            last_tick: Instant::now(),
        })
    }

    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick);
        self.last_tick = now;
        dt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClockError {
    InvalidFps { fps: u32 },
}

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClockError::InvalidFps { fps } => write!(f, "invalid fps: {fps}"),
        }
    }
}

impl std::error::Error for ClockError {}
