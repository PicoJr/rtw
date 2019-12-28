use anyhow::Error;
use rtw::{
    AbsTime, ActiveActivity, Activity, ActivityService, CurrentActivityRepository,
    FinishedActivityRepository,
};

pub struct Service<F, C>
where
    F: FinishedActivityRepository,
    C: CurrentActivityRepository,
{
    finished: F,
    current: C,
}

impl<F, C> Service<F, C>
where
    F: FinishedActivityRepository,
    C: CurrentActivityRepository,
{
    pub fn new(finished: F, current: C) -> Self {
        Service { finished, current }
    }
}

impl<F, C> ActivityService for Service<F, C>
where
    F: FinishedActivityRepository,
    C: CurrentActivityRepository,
{
    fn get_current_activity(&self) -> anyhow::Result<Option<ActiveActivity>> {
        self.current.get_current_activity()
    }

    fn start_activity(&mut self, activity: ActiveActivity) -> anyhow::Result<ActiveActivity> {
        self.stop_current_activity(activity.start_time)?;
        let started = ActiveActivity::new(activity.start_time, activity.tags);
        self.current.set_current_activity(started.clone())?;
        Ok(started)
    }

    fn stop_current_activity(&mut self, time: AbsTime) -> anyhow::Result<Option<Activity>> {
        let current = self.current.get_current_activity()?;
        match current {
            None => Ok(None),
            Some(current_activity) => {
                self.finished
                    .write_activity(current_activity.clone().into_activity(time)?)?;
                self.current.reset_current_activity()?;
                Ok(Some(current_activity.into_activity(time)?))
            }
        }
    }

    fn filter_activities<P>(&self, p: P) -> Result<Vec<Activity>, Error>
    where
        P: Fn(&Activity) -> bool,
    {
        self.finished.filter_activities(p)
    }

    fn get_activities_within(
        &self,
        range_start: AbsTime,
        range_end: AbsTime,
    ) -> Result<Vec<Activity>, Error> {
        self.filter_activities(|a| {
            range_start <= a.get_start_time() && a.get_stop_time() <= range_end
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::chrono_clock::ChronoClock;
    use crate::json_current::JsonCurrentActivityRepository;
    use crate::json_finished::JsonFinishedActivityRepository;
    use crate::ram_current::RAMCurrentActivityRepository;
    use crate::ram_finished::RAMFinishedActivityRepository;
    use crate::service::Service;
    use rtw::{ActiveActivity, ActivityService, Clock};
    use tempfile::{tempdir, TempDir};

    fn build_ram_service() -> Service<RAMFinishedActivityRepository, RAMCurrentActivityRepository> {
        Service::new(
            RAMFinishedActivityRepository::default(),
            RAMCurrentActivityRepository::default(),
        )
    }

    #[test]
    fn test_ram_ram_no_activity() {
        let service = build_ram_service();
        assert!(service.get_current_activity().unwrap().is_none());
    }

    #[test]
    fn test_ram_ram_start_activity() {
        let clock = ChronoClock {};
        let mut service = build_ram_service();
        assert!(service
            .start_activity(ActiveActivity {
                start_time: clock.get_time(),
                tags: vec![String::from("a")],
            })
            .is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
    }

    #[test]
    fn test_ram_ram_stop_activity_with_active() {
        let clock = ChronoClock {};
        let mut service = build_ram_service();
        assert!(service
            .start_activity(ActiveActivity {
                start_time: clock.get_time(),
                tags: vec![String::from("a")],
            })
            .is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
    }

    #[test]
    fn test_ram_ram_start_stop_start() {
        let clock = ChronoClock {};
        let mut service = build_ram_service();
        let start_0 = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        assert!(start_0.is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
        let stop = service.stop_current_activity(clock.get_time());
        assert!(stop.is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
        let start_1 = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("b")],
        });
        assert!(start_1.is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
    }

    fn build_ram_json_service(
        test_dir: &TempDir,
    ) -> Service<RAMFinishedActivityRepository, JsonCurrentActivityRepository> {
        let repository_path = test_dir.path().join(".rtwr.json");
        Service::new(
            RAMFinishedActivityRepository::default(),
            JsonCurrentActivityRepository::new(repository_path),
        )
    }

    #[test]
    fn test_ram_json_no_activity() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_ram_json_service(&test_dir);
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
    }

    #[test]
    fn test_ram_json_start_activity() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_ram_json_service(&test_dir);
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        let start = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        start.unwrap();
        let current = service.get_current_activity();
        assert!(current.is_ok());
        assert!(current.unwrap().is_some());
    }

    #[test]
    fn test_ram_json_stop_activity_with_active() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_ram_json_service(&test_dir);
        let start = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        start.unwrap();
        assert!(service.get_current_activity().unwrap().is_some());
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
    }

    #[test]
    fn test_ram_json_start_stop_start() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_ram_json_service(&test_dir);
        let start_0 = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        assert!(start_0.is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
        let stop = service.stop_current_activity(clock.get_time());
        assert!(stop.is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
        let start_1 = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("b")],
        });
        assert!(start_1.is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
    }

    fn build_json_service(
        test_dir: &TempDir,
    ) -> Service<JsonFinishedActivityRepository, JsonCurrentActivityRepository> {
        let writer_path = test_dir.path().join(".rtww.json");
        let repository_path = test_dir.path().join(".rtwr.json");
        Service::new(
            JsonFinishedActivityRepository::new(writer_path),
            JsonCurrentActivityRepository::new(repository_path),
        )
    }

    #[test]
    fn test_json_json_no_activity() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
    }

    #[test]
    fn test_json_json_start_activity() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        let start = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        start.unwrap();
        let current = service.get_current_activity();
        assert!(current.is_ok());
        assert!(current.unwrap().is_some());
    }

    #[test]
    fn test_json_json_stop_activity_with_active() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let start = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        start.unwrap();
        assert!(service.get_current_activity().unwrap().is_some());
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
    }

    #[test]
    fn test_json_json_start_stop_start() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let start_0 = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        assert!(start_0.is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
        let stop = service.stop_current_activity(clock.get_time());
        assert!(stop.is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
        let start_1 = service.start_activity(ActiveActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("b")],
        });
        assert!(start_1.is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
    }

    #[test]
    fn test_json_json_summary_nothing() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let (today_start, today_end) = clock.today_range();
        let activities = service.get_activities_within(today_start, today_end);
        assert!(activities.is_ok());
    }

    #[test]
    fn test_json_json_summary_something() {
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let today = chrono::Local::today();
        let range_start = today.and_hms(8, 0, 0);
        let activity_start = today.and_hms(8, 30, 0);
        let activity_end = today.and_hms(8, 45, 0);
        let range_end = today.and_hms(9, 0, 0);
        let _start = service.start_activity(ActiveActivity::new(
            activity_start.into(),
            vec![String::from("a")],
        ));
        let _stop = service.stop_current_activity(activity_end.into());
        let activities = service.get_activities_within(range_start.into(), range_end.into());
        assert!(!activities.unwrap().is_empty());
    }

    #[test]
    fn test_json_json_summary_not_in_range() {
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let today = chrono::Local::today();
        let range_start = today.and_hms(9, 0, 0);
        let activity_start = today.and_hms(8, 30, 0);
        let activity_end = today.and_hms(8, 45, 0);
        let range_end = today.and_hms(10, 0, 0);
        let _start = service.start_activity(ActiveActivity::new(
            activity_start.into(),
            vec![String::from("a")],
        ));
        let _stop = service.stop_current_activity(activity_end.into());
        let activities = service.get_activities_within(range_start.into(), range_end.into());
        assert!(activities.unwrap().is_empty());
    }
}
