use crate::rtw_core::activity::OngoingActivity;
use crate::rtw_core::repository::CurrentActivityRepository;
use anyhow::Context;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct JsonCurrentActivityRepository {
    repository_path: PathBuf,
}

impl JsonCurrentActivityRepository {
    pub fn new(path: PathBuf) -> Self {
        JsonCurrentActivityRepository {
            repository_path: path,
        }
    }
}

impl CurrentActivityRepository for JsonCurrentActivityRepository {
    fn get_current_activity(&self) -> anyhow::Result<Option<OngoingActivity>> {
        if !Path::exists(&self.repository_path) {
            Ok(None)
        } else {
            let file = File::open(&self.repository_path)?;
            let current_activity: OngoingActivity = serde_json::from_reader(file)?;
            Ok(Some(current_activity))
        }
    }

    fn set_current_activity(&mut self, activity: OngoingActivity) -> anyhow::Result<()> {
        if Path::exists(&self.repository_path) {
            fs::remove_file(&self.repository_path)?
        }
        let file = File::create(&self.repository_path)?;
        serde_json::to_writer(file, &activity)?;
        Ok(())
    }

    fn reset_current_activity(&mut self) -> anyhow::Result<()> {
        if Path::exists(&self.repository_path) {
            fs::remove_file(&self.repository_path).context("removing current activity")?
        }
        Ok(())
    }
}
