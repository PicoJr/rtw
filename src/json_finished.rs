use anyhow::{Context, Error};
use rtw::{Activity, ActivityId, FinishedActivityRepository};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

type Activities = Vec<Activity>;

pub struct JsonFinishedActivityRepository {
    writer_path: PathBuf,
}

impl JsonFinishedActivityRepository {
    pub fn new(path: PathBuf) -> Self {
        JsonFinishedActivityRepository { writer_path: path }
    }

    fn get_finished_activities(&self) -> Result<Vec<Activity>, Error> {
        if Path::exists(&self.writer_path) {
            let mut activities: Activities = vec![];
            let file = OpenOptions::new()
                .read(true)
                .write(false)
                .open(&self.writer_path)
                .context(format!("reading activity file: {:?}", &self.writer_path))?;
            let finished_activities: Activities = serde_json::from_reader(file)?;
            for finished_activity in finished_activities {
                activities.push(finished_activity);
            }
            Ok(activities)
        } else {
            Ok(vec![])
        }
    }
}

impl FinishedActivityRepository for JsonFinishedActivityRepository {
    fn write_activity(&mut self, activity: Activity) -> Result<(), Error> {
        if !Path::exists(&self.writer_path) {
            let file =
                File::create(&self.writer_path).context("creating finished activity file")?;
            let activities: Activities = vec![activity];
            serde_json::to_writer(file, &activities)?;
            Ok(())
        } else {
            let mut activities = self.get_finished_activities()?;
            activities.push(activity);
            let file = OpenOptions::new().write(true).open(&self.writer_path)?;
            serde_json::to_writer(file, &activities)?;
            Ok(())
        }
    }

    fn filter_activities<P>(&self, p: P) -> Result<Vec<(ActivityId, Activity)>, Error>
    where
        P: Fn(&(ActivityId, Activity)) -> bool,
    {
        let mut finished_activities = self.get_finished_activities()?;
        finished_activities.sort();
        let indexed_finished_activities = (0..finished_activities.len())
            .rev()
            .zip(finished_activities);
        let filtered = indexed_finished_activities.filter(p);
        Ok(filtered.collect())
    }
}
