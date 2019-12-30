//! Command-line interface (CLI) time tracker.
//!
//! This project is for educational purpose only.
//!
//! It is a _partial_ Rust implementation of [Timewarrior](https://github.com/GothenburgBitFactory/timewarrior).
//!
//! For a stable feature-rich CLI time tracker, please use Timewarrior: <https://timewarrior.net/>.

use anyhow::anyhow;
use chrono::{DateTime, Local};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
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
/// `ActivityId` = `usize`
pub type ActivityId = usize;

/// Newtype on `chrono::Date<Local>`
///
/// Date is given in local time for convenience
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DateTimeW(DateTime<Local>);

/// Convert from `DateTime<Local>` to `DateTimeW`
impl Into<DateTimeW> for DateTime<Local> {
    fn into(self) -> DateTimeW {
        DateTimeW(self)
    }
}

impl std::ops::Sub for DateTimeW {
    type Output = DurationW;

    fn sub(self, rhs: Self) -> Self::Output {
        DurationW(self.0 - rhs.0)
    }
}

impl std::fmt::Display for DateTimeW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0.format(DATETIME_FMT))
    }
}

/// Newtype on `chrono::Duration`
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

/// Time (absolute or relative)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Time {
    /// Now, can be converted to `DateTimeW` using `Clock.date_time`
    Now,
    /// MinutesAgo, can be converted to `DateTimeW` using `Clock.date_time`
    MinutesAgo(usize),
    DateTime(DateTimeW),
}

/// Clock Abstraction
pub trait Clock {
    /// Get current local time
    fn get_time(&self) -> DateTimeW;
    /// Convert a `Time` to absolute time
    ///
    /// `clock.date_time(Time::Now)` equals approximately clock.get_time();
    fn date_time(&self, time: Time) -> DateTimeW;

    /// Get time range for today
    ///
    /// today: 00:00:00 - 23:59:59
    fn today_range(&self) -> (DateTimeW, DateTimeW);

    /// Get time range for yesterday
    ///
    /// yesterday: 00:00:00 - 23:59:59
    fn yesterday_range(&self) -> (DateTimeW, DateTimeW);

    /// Get time range for last week
    ///
    /// last week (ISO 8601, week start on monday)
    ///
    /// last week: monday: 00:00:00 - sunday: 23:59:59
    fn last_week_range(&self) -> (DateTimeW, DateTimeW);
}

/// A finished activity (with a stop time)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    /// Activity start time
    start_time: DateTimeW,
    /// Activity `stop time` >= `start time`
    stop_time: DateTimeW,
    /// Activity tags
    tags: Tags,
}

impl Activity {
    /// start time getter
    pub fn get_start_time(&self) -> DateTimeW {
        self.start_time
    }
    /// stop time getter
    pub fn get_stop_time(&self) -> DateTimeW {
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
    /// Return tags
    pub fn get_tags(&self) -> Tags {
        self.tags.clone()
    }
}

/// Activities are sorted by start time
impl Ord for Activity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_start_time().cmp(&other.get_start_time())
    }
}

impl PartialOrd for Activity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A started and unfinished activity (no stop time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OngoingActivity {
    /// start time
    pub start_time: DateTimeW,
    /// Activity tags
    pub tags: Tags,
}

impl OngoingActivity {
    /// Constructor
    pub fn new(start_time: DateTimeW, tags: Tags) -> Self {
        OngoingActivity { start_time, tags }
    }
    /// Start time getter
    pub fn get_start_time(&self) -> DateTimeW {
        self.start_time
    }
    /// Return title (activity tags joined by a space)
    ///
    /// ```
    /// # use rtw::{OngoingActivity, DateTimeW};
    /// let activity = OngoingActivity::new(chrono::Local::now().into(), vec![String::from("foo"), String::from("bar")]);
    /// assert_eq!(activity.get_title(), "foo bar");
    /// ```
    pub fn get_title(&self) -> String {
        self.tags.join(" ")
    }
    /// Convert active activity to finished activity
    ///
    /// ```
    /// # use rtw::{OngoingActivity, DateTimeW, Activity};
    /// let activity = OngoingActivity::new(chrono::Local::now().into(), vec![String::from("foo"), String::from("bar")]);
    /// let finished: Activity = activity.into_activity(chrono::Local::now().into()).unwrap();
    /// ```
    /// `stop_time` should be >= `start_time` otherwise error
    pub fn into_activity(self, stop_time: DateTimeW) -> anyhow::Result<Activity> {
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

/// A service for activities
///
/// Abstracts activities queries and modifications
pub trait ActivityService {
    /// Get current activity if any
    ///
    /// May fail depending on backend implementation
    fn get_current_activity(&self) -> anyhow::Result<Option<OngoingActivity>>;
    /// Start a new activity
    ///
    /// May fail depending on backend implementation
    ///
    /// Returns new current activity
    fn start_activity(&mut self, activity: OngoingActivity) -> anyhow::Result<OngoingActivity>;
    /// Stop current activity
    ///
    /// May fail depending on backend implementation
    ///
    /// Returns stopped activity if any
    fn stop_current_activity(&mut self, time: DateTimeW) -> anyhow::Result<Option<Activity>>;
    /// Filter finished activities
    ///
    /// May fail depending on implementation
    ///
    /// Returns finished activities sorted by start date
    ///
    /// ActivityId: 0 <=> last finished activity
    fn filter_activities<P>(&self, p: P) -> anyhow::Result<Vec<(ActivityId, Activity)>>
    where
        P: Fn(&(ActivityId, Activity)) -> bool;
    /// Delete activity with id
    ///
    /// May fail depending on implementation
    ///
    /// Returns deleted activity if successful
    fn delete_activity(&self, id: ActivityId) -> anyhow::Result<Option<Activity>>;
    /// Track a finished activity
    ///
    /// May fail depending on backend implementation
    ///
    /// Returns tracked activity if successful
    fn track_activity(&mut self, activity: Activity) -> anyhow::Result<Activity>;
}

/// A service for persisting and querying finished activities
///
/// Abstracts finished activities queries and persistence
pub trait FinishedActivityRepository {
    /// Write finished activity
    ///
    /// May fail depending on backend implementation
    fn write_activity(&mut self, activity: Activity) -> anyhow::Result<()>;
    /// Filter finished activities
    ///
    /// May fail depending on implementation
    ///
    /// Returns finished activities sorted by start date
    ///
    /// ActivityId: 0 <=> last finished activity
    fn filter_activities<P>(&self, p: P) -> anyhow::Result<Vec<(ActivityId, Activity)>>
    where
        P: Fn(&(ActivityId, Activity)) -> bool;
    /// Delete activity with id
    ///
    /// May fail depending on implementation
    ///
    /// Returns deleted activity if successful
    fn delete_activity(&self, id: ActivityId) -> anyhow::Result<Option<Activity>>;
}

/// A service for persisting and querying current activity
///
/// Abstracts current activity queries and persistence
pub trait CurrentActivityRepository {
    /// Retrieve current activity if any
    ///
    /// May fail depending on backend implementation
    fn get_current_activity(&self) -> anyhow::Result<Option<OngoingActivity>>;
    /// Set `activity` as current activity
    ///
    /// May fail depending on backend implementation
    fn set_current_activity(&mut self, activity: OngoingActivity) -> anyhow::Result<()>;
    /// Reset current activity to none
    ///
    /// After calling this function, get_current_activity should return None
    fn reset_current_activity(&mut self) -> anyhow::Result<()>;
}
