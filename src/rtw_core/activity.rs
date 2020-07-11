//! Activity and OngoingActivity

use crate::rtw_core::datetimew::DateTimeW;
use crate::rtw_core::durationw::DurationW;
use crate::rtw_core::{Description, Tags};
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
    #[serde(default)]
    description: Option<Description>,
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

    /// Return Description
    pub fn get_description(&self) -> Option<Description> {
        self.description.clone()
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
    #[serde(default)]
    pub description: Option<Description>,
}

impl OngoingActivity {
    /// Constructor
    pub fn new(start_time: DateTimeW, tags: Tags, description: Option<Description>) -> Self {
        OngoingActivity {
            start_time,
            tags,
            description,
        }
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
                description: self.description,
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

/// Check intersection between a finished activity and a date
///
/// Returns Some(activity) if it intersects else None.
pub fn intersect(finished: &Activity, datetimew: &DateTimeW) -> Option<Activity> {
    if (&finished.start_time < datetimew) && (datetimew < &finished.stop_time) {
        Some(finished.clone())
    } else {
        None
    }
}

/// Check overlap between 2 finished activities
///
/// Returns Some(first) if first activity overlaps with the second else None.
pub fn overlap(finished: &Activity, other: &Activity) -> Option<Activity> {
    if finished < other {
        intersect(finished, &other.start_time)
    } else {
        intersect(other, &finished.start_time).map(|_| finished.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::rtw_core::activity::{intersect, overlap, Activity};
    use chrono::{Local, TimeZone};

    #[test]
    fn test_intersect() {
        let finished = Activity {
            start_time: Local
                .datetime_from_str("2020-12-25T09:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            stop_time: Local
                .datetime_from_str("2020-12-25T10:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            tags: vec![],
            description: None,
        };
        let date = Local
            .datetime_from_str("2020-12-25T09:30:00", "%Y-%m-%dT%H:%M:%S")
            .unwrap()
            .into();
        assert!(intersect(&finished, &date).is_some());
        let date = Local
            .datetime_from_str("2020-12-25T10:30:00", "%Y-%m-%dT%H:%M:%S")
            .unwrap()
            .into();
        assert!(intersect(&finished, &date).is_none());
    }

    #[test]
    fn test_overlap() {
        let finished = Activity {
            start_time: Local
                .datetime_from_str("2020-12-25T09:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            stop_time: Local
                .datetime_from_str("2020-12-25T10:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            tags: vec![],
            description: None,
        };
        let other = Activity {
            start_time: Local
                .datetime_from_str("2020-12-25T09:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            stop_time: Local
                .datetime_from_str("2020-12-25T11:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            tags: vec![],
            description: None,
        };
        assert!(overlap(&finished, &other).is_some());
        let other = Activity {
            start_time: Local
                .datetime_from_str("2020-12-25T08:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            stop_time: Local
                .datetime_from_str("2020-12-25T09:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            tags: vec![],
            description: None,
        };
        assert!(overlap(&finished, &other).is_some());
        let other = Activity {
            start_time: Local
                .datetime_from_str("2020-12-25T08:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            stop_time: Local
                .datetime_from_str("2020-12-25T10:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            tags: vec![],
            description: None,
        };
        assert!(overlap(&finished, &other).is_some());
        let other = Activity {
            start_time: Local
                .datetime_from_str("2020-12-25T09:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            stop_time: Local
                .datetime_from_str("2020-12-25T09:45:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            tags: vec![],
            description: None,
        };
        assert!(overlap(&finished, &other).is_some());
        let other = Activity {
            start_time: Local
                .datetime_from_str("2020-12-25T10:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            stop_time: Local
                .datetime_from_str("2020-12-25T11:45:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
                .into(),
            tags: vec![],
            description: None,
        };
        assert!(overlap(&finished, &other).is_none());
    }
}
