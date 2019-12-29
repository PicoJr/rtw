use anyhow::Error;
use rtw::{Activity, ActivityId, FinishedActivityRepository};

pub struct RAMFinishedActivityRepository {
    activities: Vec<Activity>,
}

impl RAMFinishedActivityRepository {
    pub fn default() -> Self {
        RAMFinishedActivityRepository { activities: vec![] }
    }
}

impl FinishedActivityRepository for RAMFinishedActivityRepository {
    fn write_activity(&mut self, activity: Activity) -> anyhow::Result<()> {
        self.activities.push(activity);
        Ok(())
    }

    fn filter_activities<P>(&self, p: P) -> Result<Vec<(ActivityId, Activity)>, Error>
    where
        P: Fn(&(ActivityId, Activity)) -> bool,
    {
        unimplemented!()
    }
}
