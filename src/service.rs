use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::datetimew::DateTimeW;
use crate::rtw_core::service::ActivityService;
use crate::rtw_core::storage::Storage;
use crate::rtw_core::ActivityId;

pub struct Service<S>
where
    S: Storage,
{
    storage: S,
}

impl<S> Service<S>
where
    S: Storage,
{
    pub fn new(storage: S) -> Self {
        Service { storage }
    }
}

impl<S> ActivityService for Service<S>
where
    S: Storage,
{
    fn get_current_activity(&self) -> anyhow::Result<Option<OngoingActivity>> {
        self.storage.get_current_activity().map_err(|e| e.into())
    }

    fn start_activity(&mut self, activity: OngoingActivity) -> anyhow::Result<OngoingActivity> {
        self.stop_current_activity(activity.start_time)?;
        let started = OngoingActivity::new(activity.start_time, activity.tags);
        self.storage.set_current_activity(started.clone())?;
        Ok(started)
    }

    fn stop_current_activity(&mut self, time: DateTimeW) -> anyhow::Result<Option<Activity>> {
        let current = self.storage.get_current_activity()?;
        match current {
            None => Ok(None),
            Some(current_activity) => {
                self.storage
                    .write_activity(current_activity.clone().into_activity(time)?)?;
                self.storage.reset_current_activity()?;
                Ok(Some(current_activity.into_activity(time)?))
            }
        }
    }

    fn filter_activities<P>(&self, p: P) -> anyhow::Result<Vec<(ActivityId, Activity)>>
    where
        P: Fn(&(ActivityId, Activity)) -> bool,
    {
        self.storage.filter_activities(p).map_err(|e| e.into())
    }

    fn get_finished_activities(&self) -> anyhow::Result<Vec<(ActivityId, Activity)>> {
        self.storage.get_finished_activities().map_err(|e| e.into())
    }

    fn delete_activity(&self, id: ActivityId) -> anyhow::Result<Option<Activity>> {
        self.storage.delete_activity(id).map_err(|e| e.into())
    }

    fn track_activity(&mut self, activity: Activity) -> anyhow::Result<Activity> {
        self.storage.write_activity(activity.clone())?;
        Ok(activity)
    }
}

#[cfg(test)]
mod tests {
    use crate::chrono_clock::ChronoClock;
    use crate::json_storage::JsonStorage;
    use crate::rtw_core::activity::OngoingActivity;
    use crate::rtw_core::clock::Clock;
    use crate::rtw_core::datetimew::DateTimeW;
    use crate::rtw_core::service::ActivityService;
    use crate::service::Service;
    use tempfile::{tempdir, TempDir};

    fn build_json_service(test_dir: &TempDir) -> Service<JsonStorage> {
        let finished_path = test_dir.path().join(".rtwh.json");
        let current_path = test_dir.path().join(".rtwc.json");
        Service::new(JsonStorage::new(current_path, finished_path))
    }

    #[test]
    fn test_no_activity() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
    }

    #[test]
    fn test_start_activity() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        let start = service.start_activity(OngoingActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        start.unwrap();
        let current = service.get_current_activity();
        assert!(current.is_ok());
        assert!(current.unwrap().is_some());
    }

    #[test]
    fn test_stop_activity_with_active() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let start = service.start_activity(OngoingActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        start.unwrap();
        assert!(service.get_current_activity().unwrap().is_some());
        assert!(service.stop_current_activity(clock.get_time()).is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
    }

    #[test]
    fn test_start_stop_start() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let start_0 = service.start_activity(OngoingActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("a")],
        });
        assert!(start_0.is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
        let stop = service.stop_current_activity(clock.get_time());
        assert!(stop.is_ok());
        assert!(service.get_current_activity().unwrap().is_none());
        let start_1 = service.start_activity(OngoingActivity {
            start_time: clock.get_time(),
            tags: vec![String::from("b")],
        });
        assert!(start_1.is_ok());
        assert!(service.get_current_activity().unwrap().is_some());
    }

    #[test]
    fn test_summary_nothing() {
        let clock = ChronoClock {};
        let test_dir = tempdir().expect("error while creating tempdir");
        let service = build_json_service(&test_dir);
        let (today_start, today_end) = clock.today_range();
        let activities = service.filter_activities(|(_id, a)| {
            today_start <= a.get_start_time() && a.get_start_time() <= today_end
        });
        assert!(activities.is_ok());
    }

    #[test]
    fn test_summary_something() {
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let today = chrono::Local::today();
        let range_start: DateTimeW = today.and_hms(8, 0, 0).into();
        let activity_start: DateTimeW = today.and_hms(8, 30, 0).into();
        let activity_end: DateTimeW = today.and_hms(8, 45, 0).into();
        let range_end: DateTimeW = today.and_hms(9, 0, 0).into();
        let _start = service.start_activity(OngoingActivity::new(
            activity_start,
            vec![String::from("a")],
        ));
        let _stop = service.stop_current_activity(activity_end);
        let activities = service.filter_activities(|(_id, a)| {
            range_start <= a.get_start_time() && a.get_start_time() <= range_end
        });
        assert!(!activities.unwrap().is_empty());
    }

    #[test]
    fn test_summary_not_in_range() {
        let test_dir = tempdir().expect("error while creating tempdir");
        let mut service = build_json_service(&test_dir);
        let today = chrono::Local::today();
        let range_start: DateTimeW = today.and_hms(9, 0, 0).into();
        let activity_start: DateTimeW = today.and_hms(8, 30, 0).into();
        let activity_end: DateTimeW = today.and_hms(8, 45, 0).into();
        let range_end: DateTimeW = today.and_hms(10, 0, 0).into();
        let _start = service.start_activity(OngoingActivity::new(
            activity_start,
            vec![String::from("a")],
        ));
        let _stop = service.stop_current_activity(activity_end);
        let activities = service.filter_activities(|(_id, a)| {
            range_start <= a.get_start_time() && a.get_start_time() <= range_end
        });
        assert!(activities.unwrap().is_empty());
    }
}
