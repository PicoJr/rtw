//! A simplified TimeWarrior client written in Rust.
//!
//! This project is for learning purpose only.
//! If you need to track your time consider using TimeWarrior.
//!
//! Dates and Time are provided by the `chrono` crate

use anyhow::anyhow;
use chrono::{DateTime, Local};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Absolute dates are parsed and displayed using this format
///
/// e.g. 2019-12-25T18:43:00
pub const DATETIME_FMT: &str = "%Y-%m-%dT%H:%M:%S";
pub const RELATIVE_TIME_REGEX: &str = r"(\d+)m";

/// `Tag` = `String`
pub type Tag = String;
/// `Tags` = `Vec<Tag>`
pub type Tags = Vec<Tag>;

/// Absolute Time ie a date
///
/// Date is given in local time for convenience
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AbsTime(DateTime<Local>);

/// Convert from `DateTime<Local>` to `AbsTime`
impl Into<AbsTime> for DateTime<Local> {
    fn into(self) -> AbsTime {
        AbsTime(self)
    }
}

impl std::ops::Sub for AbsTime {
    type Output = DurationW;

    fn sub(self, rhs: Self) -> Self::Output {
        DurationW(self.0 - rhs.0)
    }
}

impl std::fmt::Display for AbsTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0.format(DATETIME_FMT))
    }
}

/// New Type on `chrono::Duration`
pub struct DurationW(chrono::Duration);

impl fmt::Display for DurationW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.0.num_hours(),
            self.0.num_minutes(),
            self.0.num_seconds()
        )
    }
}

/// Time
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Time {
    /// Now, can be converted to `AbsTime` using `Clock.abs_time`
    Now,
    MinutesAgo(usize),
    Abs(AbsTime),
}

/// Clock Abstraction
pub trait Clock {
    /// Get current local time
    fn get_time(&self) -> AbsTime;
    /// Convert Time to absolute time
    ///
    /// `clock.abs_time(Time::Now)` equals approximately clock.get_time();
    fn abs_time(&self, time: Time) -> AbsTime;

    /// Get time range for today
    ///
    /// today: 00:00:00 - 23:59:59
    fn today_range(&self) -> (AbsTime, AbsTime);

    /// Get time range for yesterday
    ///
    /// yesterday: 00:00:00 - 23:59:59
    fn yesterday_range(&self) -> (AbsTime, AbsTime);
}

/// A finished Activity (with a stop_time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    /// Activity start time
    start_time: AbsTime,
    /// Activity `stop time` >= `start time`
    stop_time: AbsTime,
    /// Activity tags
    tags: Tags,
}

impl Activity {
    /// start time getter
    pub fn get_start_time(&self) -> AbsTime {
        self.start_time
    }
    /// stop time getter
    pub fn get_stop_time(&self) -> AbsTime {
        self.stop_time
    }
    /// Return activity duration
    pub fn get_duration(&self) -> DurationW {
        self.stop_time - self.start_time
    }
    /// Return activity title (its tags joined by a space)
    pub fn get_title(&self) -> String {
        self.tags.join(" ")
    }
}

/// A started and unfinished Activity (no stop time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveActivity {
    /// start time
    pub start_time: AbsTime,
    /// Activity tags
    pub tags: Tags,
}

impl ActiveActivity {
    /// Constructor
    pub fn new(start_time: AbsTime, tags: Tags) -> Self {
        ActiveActivity { start_time, tags }
    }
    /// Start time getter
    pub fn get_start_time(&self) -> AbsTime {
        self.start_time
    }
    /// Return title (activity tags joined by a space)
    pub fn get_title(&self) -> String {
        self.tags.join(" ")
    }
    /// Convert active activity to finished activity
    ///
    /// `stop_time` should be >= `start_time` otherwise error
    pub fn into_activity(self, stop_time: AbsTime) -> anyhow::Result<Activity> {
        if self.start_time <= stop_time {
            Ok(Activity {
                start_time: self.start_time,
                stop_time,
                tags: self.tags,
            })
        } else {
            Err(anyhow!(
                "stop time ({}) < start_time ({})",
                stop_time,
                self.start_time
            ))
        }
    }
}

/// A service for Activities
///
/// Abstracts activities queries and modifications
pub trait ActivityService {
    /// Get current activity if any
    ///
    /// May fail depending on backend implementation
    fn get_current_activity(&self) -> anyhow::Result<Option<ActiveActivity>>;
    /// Start a new activity
    ///
    /// May fail depending on backend implementation
    ///
    /// Returns new current activity
    fn start_activity(&mut self, activity: ActiveActivity) -> anyhow::Result<ActiveActivity>;
    /// Stop current activity
    ///
    /// May fail depending on backend implementation
    ///
    /// Returns stopped activity if any
    fn stop_current_activity(&mut self, time: AbsTime) -> anyhow::Result<Option<Activity>>;
    /// Filter finished activities
    ///
    /// May fail depending on implementation
    fn filter_activities<P>(&self, p: P) -> anyhow::Result<Vec<Activity>>
    where
        P: Fn(&Activity) -> bool;
    /// Get finished activities within time range
    ///
    /// May fail depending on backed implementation
    ///
    /// Returns activities within time range
    ///
    /// all activities such that range_start <= activity start <= range_end
    fn get_activities_within(
        &self,
        range_start: AbsTime,
        range_end: AbsTime,
    ) -> anyhow::Result<Vec<Activity>>;
}

/// A service for persisting finished activities
///
/// Abstracts finished activities persistence
pub trait FinishedActivityRepository {
    /// Write finished activity
    ///
    /// May fail depending on backend implementation
    fn write_activity(&mut self, activity: Activity) -> anyhow::Result<()>;
    /// Filter finished activities
    ///
    /// May fail depending on implementation
    fn filter_activities<P>(&self, p: P) -> anyhow::Result<Vec<Activity>>
    where
        P: Fn(&Activity) -> bool;
}

/// A service for persisting current activity
///
/// Abstracts current activity persistence
pub trait CurrentActivityRepository {
    /// Retrieve current activity if any
    ///
    /// May fail depending on backend implementation
    fn get_current_activity(&self) -> anyhow::Result<Option<ActiveActivity>>;
    /// Set `activity` as current activity
    ///
    /// May fail depending on backend implementation
    fn set_current_activity(&mut self, activity: ActiveActivity) -> anyhow::Result<()>;
    /// Reset current activity to none
    ///
    /// After calling this function, get_current_activity should return None
    fn reset_current_activity(&mut self) -> anyhow::Result<()>;
}
