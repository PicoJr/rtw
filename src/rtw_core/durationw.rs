//! Newtype on `chrono::Duration`
use chrono::Duration;
use std::fmt;
use std::fmt::{Error, Formatter};
use std::iter::Sum;
use std::ops::Add;

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

impl Default for DurationW {
    fn default() -> Self {
        DurationW::new(Duration::seconds(0))
    }
}

impl From<Duration> for DurationW {
    fn from(d: Duration) -> Self {
        DurationW(d)
    }
}

impl From<DurationW> for Duration {
    fn from(d: DurationW) -> Self {
        d.0
    }
}

impl Add<DurationW> for DurationW {
    type Output = DurationW;

    fn add(self, rhs: DurationW) -> Self::Output {
        DurationW::new(self.0 + rhs.0)
    }
}

impl Sum for DurationW {
    fn sum<I: Iterator<Item = DurationW>>(iter: I) -> Self {
        iter.fold(DurationW::default(), Add::add)
    }
}
