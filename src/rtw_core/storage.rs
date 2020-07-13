//! Storage: abstracts activities storage (file, memory...)
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
    /// Retrieve ongoing activities if any
    ///
    /// May fail depending on backend implementation
    fn get_ongoing_activities(
        &self,
    ) -> Result<Vec<(ActivityId, OngoingActivity)>, Self::StorageError>;
    /// Retrieve ongoing activity with id if any
    ///
    /// May fail depending on backend implementation
    fn get_ongoing_activity(
        &self,
        id: ActivityId,
    ) -> Result<Option<OngoingActivity>, Self::StorageError>;
    /// Add `activity` to ongoing activities
    ///
    /// May fail depending on backend implementation
    fn add_ongoing_activity(&mut self, activity: OngoingActivity)
        -> Result<(), Self::StorageError>;
    /// Remove ongoing activity
    ///
    /// May fail depending on backend implementation
    fn remove_ongoing_activity(
        &mut self,
        id: ActivityId,
    ) -> Result<Option<OngoingActivity>, Self::StorageError>;
}
