use crate::rtw_core::activity::Activity;
use crate::rtw_core::repository::FinishedActivityRepository;
use crate::rtw_core::ActivityId;
use anyhow::Context;
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

    fn get_finished_activities(&self) -> anyhow::Result<Vec<Activity>> {
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

    fn get_sorted_activities(&self) -> anyhow::Result<Vec<(ActivityId, Activity)>> {
        let mut finished_activities = self.get_finished_activities()?;
        finished_activities.sort();
        Ok((0..finished_activities.len())
            .rev()
            .zip(finished_activities)
            .collect())
    }
}

impl FinishedActivityRepository for JsonFinishedActivityRepository {
    fn write_activity(&mut self, activity: Activity) -> anyhow::Result<()> {
        if !Path::exists(&self.writer_path) {
            let file =
                File::create(&self.writer_path).context("creating finished activity file")?;
            let activities: Activities = vec![activity];
            serde_json::to_writer(file, &activities)?;
            Ok(())
        } else {
            let mut activities = self.get_finished_activities()?;
            activities.push(activity);
            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.writer_path)?;
            serde_json::to_writer(file, &activities)?;
            Ok(())
        }
    }

    fn filter_activities<P>(&self, p: P) -> anyhow::Result<Vec<(ActivityId, Activity)>>
    where
        P: Fn(&(ActivityId, Activity)) -> bool,
    {
        let indexed_finished_activities = self.get_sorted_activities()?;
        let filtered = indexed_finished_activities.into_iter().filter(p);
        Ok(filtered.collect())
    }

    fn get_finished_activities(&self) -> anyhow::Result<Vec<(usize, Activity)>> {
        self.get_sorted_activities()
    }

    fn delete_activity(&self, id: ActivityId) -> anyhow::Result<Option<Activity>> {
        let finished_activities = self.get_sorted_activities()?;
        let mut remove = Option::None;
        let mut keep: Vec<Activity> = vec![];
        for (i, a) in finished_activities {
            if i == id {
                remove = Option::Some(a);
            } else {
                keep.push(a);
            }
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.writer_path)?;
        serde_json::to_writer(file, &keep)?;
        Ok(remove)
    }
}
