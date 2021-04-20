//! Newtype on `chrono::Date<Local>`
use crate::rtw_core::durationw::DurationW;
use crate::rtw_core::DATETIME_FMT;
use chrono::{DateTime, Local};
use std::fmt::{Error, Formatter};

use serde::{Deserialize, Serialize};

/// Newtype on `chrono::Date<Local>`
///
/// Date is given in local time for convenience
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DateTimeW(DateTime<Local>);

impl From<DateTime<Local>> for DateTimeW {
    fn from(dt: DateTime<Local>) -> Self {
        DateTimeW(dt)
    }
}
impl From<DateTimeW> for DateTime<Local> {
    fn from(dt: DateTimeW) -> Self {
        dt.0
    }
}

impl std::ops::Sub for DateTimeW {
    type Output = DurationW;

    fn sub(self, rhs: Self) -> Self::Output {
        DurationW::new(self.0 - rhs.0)
    }
}

impl std::fmt::Display for DateTimeW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0.format(DATETIME_FMT))
    }
}
