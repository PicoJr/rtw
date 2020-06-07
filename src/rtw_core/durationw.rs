//! Newtype on `chrono::Duration`
use chrono::Duration;
use std::fmt;
use std::fmt::{Error, Formatter};

/// Newtype on `chrono::Duration`
pub struct DurationW(chrono::Duration);

impl fmt::Display for DurationW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.0.num_seconds() / 3600,
            (self.0.num_seconds() / 60) % 60,
            (self.0.num_seconds() % 60)
        )
    }
}

impl DurationW {
    pub fn new(d: Duration) -> Self {
        DurationW(d)
    }
}

impl From<Duration> for DurationW {
    fn from(d: Duration) -> Self {
        DurationW(d)
    }
}

impl Into<Duration> for DurationW {
    fn into(self) -> Duration {
        self.0
    }
}
