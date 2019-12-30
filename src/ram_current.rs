use rtw::{CurrentActivityRepository, OngoingActivity};

pub struct RAMCurrentActivityRepository {
    current: Option<OngoingActivity>,
}

impl RAMCurrentActivityRepository {
    pub fn default() -> Self {
        RAMCurrentActivityRepository { current: None }
    }
}

impl CurrentActivityRepository for RAMCurrentActivityRepository {
    fn get_current_activity(&self) -> anyhow::Result<Option<OngoingActivity>> {
        Ok(self.current.clone())
    }

    fn set_current_activity(&mut self, activity: OngoingActivity) -> anyhow::Result<()> {
        self.current = Some(activity);
        Ok(())
    }

    fn reset_current_activity(&mut self) -> anyhow::Result<()> {
        self.current = None;
        Ok(())
    }
}
