use chrono::{Datelike, Duration};
use rtw::{AbsTime, Clock, Time};

pub struct ChronoClock {}

impl Clock for ChronoClock {
    fn get_time(&self) -> AbsTime {
        chrono::Local::now().into()
    }

    fn abs_time(&self, time: Time) -> AbsTime {
        match time {
            Time::Now => self.get_time(),
            Time::Abs(abs_time) => abs_time,
            Time::MinutesAgo(mins) => {
                (chrono::Local::now() - chrono::Duration::minutes(mins as i64)).into()
            }
        }
    }

    fn today_range(&self) -> (AbsTime, AbsTime) {
        let today = chrono::Local::today();
        (
            today.and_hms(0, 0, 0).into(),
            today.and_hms(23, 59, 59).into(),
        )
    }

    fn yesterday_range(&self) -> (AbsTime, AbsTime) {
        let today = chrono::Local::today();
        let yesterday = today - chrono::Duration::days(1); // so proud
        (
            yesterday.and_hms(0, 0, 0).into(),
            yesterday.and_hms(23, 59, 59).into(),
        )
    }

    fn last_week_range(&self) -> (AbsTime, AbsTime) {
        let today = chrono::Local::today();
        let weekday = today.weekday();
        let this_week_monday = today - Duration::days(weekday.num_days_from_monday() as i64);
        let last_week_monday = this_week_monday - Duration::days(7);
        let last_week_sunday = this_week_monday - Duration::days(1);
        (
            last_week_monday.and_hms(0, 0, 0).into(),
            last_week_sunday.and_hms(23, 59, 59).into(),
        )
    }
}
