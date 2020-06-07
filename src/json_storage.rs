use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::storage::Storage;
use crate::rtw_core::ActivityId;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use thiserror::Error;

type Activities = Vec<Activity>;
type ActivityWithId = (ActivityId, Activity);

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

    fn filter_activities<P>(&self, p: P) -> Result<Vec<(usize, Activity)>, Self::StorageError>
    where
        P: Fn(&(ActivityId, Activity)) -> bool,
    {
        let indexed_finished_activities = self.get_sorted_activities()?;
        let filtered = indexed_finished_activities.into_iter().filter(p);
        Ok(filtered.collect())
    }

    fn get_finished_activities(&self) -> Result<Vec<(usize, Activity)>, Self::StorageError> {
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

    fn get_current_activity(&self) -> Result<Option<OngoingActivity>, Self::StorageError> {
        if !Path::exists(&self.current_path) {
            Ok(None)
        } else {
            let file = File::open(&self.current_path)?;
            let current_activity: OngoingActivity = serde_json::from_reader(file)?;
            Ok(Some(current_activity))
        }
    }

    fn set_current_activity(
        &mut self,
        activity: OngoingActivity,
    ) -> Result<(), Self::StorageError> {
        if Path::exists(&self.current_path) {
            fs::remove_file(&self.current_path)?
        }
        let file = File::create(&self.current_path)?;
        serde_json::to_writer(file, &activity)?;
        Ok(())
    }

    fn reset_current_activity(&mut self) -> Result<Option<OngoingActivity>, Self::StorageError> {
        let ongoing = self.get_current_activity()?;
        if Path::exists(&self.current_path) {
            fs::remove_file(&self.current_path)?
        }
        Ok(ongoing)
    }
}
