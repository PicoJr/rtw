use anyhow::Error;
use rtw::{AbsTime, Activity, FinishedActivityRepository};

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

    fn get_activities_within(
        &self,
        range_start: AbsTime,
        range_end: AbsTime,
    ) -> Result<Vec<Activity>, Error> {
        unimplemented!()
    }
}
