use crate::rtw_core::datetimew::DateTimeW;
use crate::rtw_core::durationw::DurationW;
use crate::rtw_core::Tags;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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
    pub fn get_title(&self) -> String {
        self.tags.join(" ")
    }
    /// Convert active activity to finished activity
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

pub fn intersect(finished: &Activity, datetimew: &DateTimeW) -> Option<Activity> {
    if (&finished.start_time < datetimew) && (datetimew < &finished.stop_time) {
        Some(finished.clone())
    } else {
        None
    }
}
