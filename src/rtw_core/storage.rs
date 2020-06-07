use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::ActivityId;
use std::error::Error;

pub trait Storage {
    // see anyhow::Error type constraints
    type StorageError: Error + Sync + Send + 'static;

    /// Write finished activity
    ///
    /// May fail depending on backend implementation
    fn write_activity(&mut self, activity: Activity) -> Result<(), Self::StorageError>;
    /// Filter finished activities
    ///
    /// May fail depending on implementation
    ///
    /// Returns finished activities sorted by start date
    ///
    /// ActivityId: 0 <=> last finished activity
    fn filter_activities<P>(&self, p: P) -> Result<Vec<(ActivityId, Activity)>, Self::StorageError>
    where
        P: Fn(&(ActivityId, Activity)) -> bool;
    /// Get all finished activities
    ///
    /// May fail depending on implementation
    ///
    /// Returns finished activities sorted by start date
    ///
    /// ActivityId: 0 <=> last finished activity
    fn get_finished_activities(&self) -> Result<Vec<(ActivityId, Activity)>, Self::StorageError>;
    /// Delete activity with id
    ///
    /// May fail depending on implementation
    ///
    /// Returns deleted activity if successful
    fn delete_activity(&self, id: ActivityId) -> Result<Option<Activity>, Self::StorageError>;
    /// Retrieve current activity if any
    ///
    /// May fail depending on backend implementation
    fn get_current_activity(&self) -> Result<Option<OngoingActivity>, Self::StorageError>;
    /// Set `activity` as current activity
    ///
    /// May fail depending on backend implementation
    fn set_current_activity(&mut self, activity: OngoingActivity)
        -> Result<(), Self::StorageError>;
    /// Reset current activity to none
    ///
    /// After calling this function, get_current_activity should return None
    fn reset_current_activity(&mut self) -> Result<Option<OngoingActivity>, Self::StorageError>;
}
