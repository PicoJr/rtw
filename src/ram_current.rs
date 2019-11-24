use rtw::{ActiveActivity, CurrentActivityRepository};

pub struct RAMCurrentActivityRepository {
    current: Option<ActiveActivity>,
}

impl RAMCurrentActivityRepository {
    pub fn default() -> Self {
        RAMCurrentActivityRepository { current: None }
    }
}

impl CurrentActivityRepository for RAMCurrentActivityRepository {
    fn get_current_activity(&self) -> anyhow::Result<Option<ActiveActivity>> {
        Ok(self.current.clone())
    }

    fn set_current_activity(&mut self, activity: ActiveActivity) -> anyhow::Result<()> {
        self.current = Some(activity);
        Ok(())
    }

    fn reset_current_activity(&mut self) -> anyhow::Result<()> {
        self.current = None;
        Ok(())
    }
}
