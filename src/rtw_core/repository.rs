use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::ActivityId;

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
