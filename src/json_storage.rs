//! Store activities (current, finished) as Json files.
use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::storage::Storage;
use crate::rtw_core::ActivityId;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use thiserror::Error;

type Activities = Vec<Activity>;
type ActivityWithId = (ActivityId, Activity);
type OngoingActivityWithId = (ActivityId, OngoingActivity);

#[derive(Error, Debug)]
pub enum JsonStorageError {
    #[error("storage io error")]
    IOError(#[from] std::io::Error),
    #[error("(de)serialization failed")]
    SerdeJsonError(#[from] serde_json::error::Error),
}

pub struct JsonStorage {
    current_path: PathBuf,
    finished_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OngoingActivities {
    ongoing: Vec<OngoingActivity>,
}

impl JsonStorage {
    pub fn new(current_path: PathBuf, finished_path: PathBuf) -> Self {
        JsonStorage {
            current_path,
            finished_path,
        }
    }

    fn get_finished_activities(&self) -> Result<Vec<Activity>, JsonStorageError> {
        if Path::exists(&self.finished_path) {
            let file = OpenOptions::new()
                .read(true)
                .write(false)
                .open(&self.finished_path)?;
            let finished_activities: Activities = serde_json::from_reader(file)?;
            Ok(finished_activities)
        } else {
            Ok(vec![])
        }
    }

    fn get_sorted_activities(&self) -> Result<Vec<(ActivityId, Activity)>, JsonStorageError> {
        let mut finished_activities = self.get_finished_activities()?;
        finished_activities.sort();
        Ok((0..finished_activities.len())
            .rev()
            .zip(finished_activities)
            .collect())
    }
}

impl Storage for JsonStorage {
    type StorageError = JsonStorageError;

    fn write_activity(&mut self, activity: Activity) -> Result<(), Self::StorageError> {
        if !Path::exists(&self.finished_path) {
            let file = File::create(&self.finished_path)?;
            let activities: Activities = vec![activity];
            serde_json::to_writer(file, &activities)?;
            Ok(())
        } else {
            let mut activities = self.get_finished_activities()?;
            activities.push(activity);
            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.finished_path)?;
            serde_json::to_writer(file, &activities)?;
            Ok(())
        }
    }

    fn filter_activities<P>(&self, p: P) -> Result<Vec<ActivityWithId>, Self::StorageError>
    where
        P: Fn(&(ActivityId, Activity)) -> bool,
    {
        let indexed_finished_activities = self.get_sorted_activities()?;
        let filtered = indexed_finished_activities.into_iter().filter(p);
        Ok(filtered.collect())
    }

    fn get_finished_activities(&self) -> Result<Vec<ActivityWithId>, Self::StorageError> {
        self.get_sorted_activities()
    }

    fn delete_activity(&self, id: usize) -> Result<Option<Activity>, Self::StorageError> {
        let finished_activities = self.get_sorted_activities()?;
        let (removed, kept): (Vec<&ActivityWithId>, Vec<&ActivityWithId>) = finished_activities
            .iter()
            .partition(|(finished_id, _)| *finished_id == id);
        let kept: Vec<&Activity> = kept.iter().map(|(_, a)| a).collect();
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.finished_path)?;
        serde_json::to_writer(file, &kept)?;
        Ok(match removed.as_slice() {
            [(_, removed)] => Some(removed.clone()),
            _ => None,
        })
    }

    fn get_ongoing_activities(&self) -> Result<Vec<OngoingActivityWithId>, Self::StorageError> {
        if !Path::exists(&self.current_path) {
            Ok(vec![])
        } else {
            let file = File::open(&self.current_path)?;
            let ongoing_activities: OngoingActivities = serde_json::from_reader(file)?;
            Ok(ongoing_activities
                .ongoing
                .iter()
                .cloned()
                .sorted()
                .enumerate()
                .collect())
        }
    }

    fn get_ongoing_activity(
        &self,
        id: ActivityId,
    ) -> Result<Option<OngoingActivity>, Self::StorageError> {
        let ongoing_activities = self.get_ongoing_activities()?;
        let ongoing = ongoing_activities
            .iter()
            .find(|(a_id, _a)| *a_id == id)
            .map(|(_a_id, a)| a);
        Ok(ongoing.cloned())
    }

    fn add_ongoing_activity(
        &mut self,
        activity: OngoingActivity,
    ) -> Result<(), Self::StorageError> {
        let ongoing_activities = self.get_ongoing_activities()?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.current_path)?;
        serde_json::to_writer(
            file,
            &OngoingActivities {
                ongoing: ongoing_activities
                    .iter()
                    .map(|(_a_id, a)| a)
                    .sorted()
                    .cloned()
                    .chain(std::iter::once(activity))
                    .collect(),
            },
        )?;
        Ok(())
    }

    fn remove_ongoing_activity(
        &mut self,
        id: ActivityId,
    ) -> Result<Option<OngoingActivity>, Self::StorageError> {
        let ongoing_activities = self.get_ongoing_activities()?;
        let (removed, kept): (Vec<OngoingActivityWithId>, Vec<OngoingActivityWithId>) =
            ongoing_activities
                .iter()
                .cloned()
                .partition(|(a_id, _a)| *a_id == id);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.current_path)?;
        let kept_without_id: Vec<OngoingActivity> =
            kept.iter().cloned().sorted().map(|(_a_id, a)| a).collect();
        serde_json::to_writer(
            file,
            &OngoingActivities {
                ongoing: kept_without_id,
            },
        )?;
        Ok(removed.first().cloned().map(|(_a_id, a)| a))
    }
}
