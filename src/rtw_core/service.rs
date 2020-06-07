//! A service for activities: abstracts activities queries and modifications.
use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::datetimew::DateTimeW;
use crate::rtw_core::ActivityId;

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
    /// Cancel current activity
    ///
    /// May fail depending on backend implementation
    ///
    /// Returns cancelled activity if any
    fn cancel_current_activity(&mut self) -> anyhow::Result<Option<OngoingActivity>>;
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
    /// Get all finished activities
    ///
    /// May fail depending on implementation
    ///
    /// Returns finished activities sorted by start date
    ///
    /// ActivityId: 0 <=> last finished activity
    fn get_finished_activities(&self) -> anyhow::Result<Vec<(ActivityId, Activity)>>;
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
