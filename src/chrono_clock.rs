use chrono::{Date, Datelike, Duration, Local};
use rtw::{Clock, DateTimeW, Time};

pub struct ChronoClock {}

impl Clock for ChronoClock {
    fn get_time(&self) -> DateTimeW {
        chrono::Local::now().into()
    }

    fn date_time(&self, time: Time) -> DateTimeW {
        match time {
            Time::Now => self.get_time(),
            Time::DateTime(abs_time) => abs_time,
            Time::MinutesAgo(mins) => {
                (chrono::Local::now() - chrono::Duration::minutes(mins as i64)).into()
            }
            _ => self.get_time(),
        }
    }

    fn today_range(&self) -> (DateTimeW, DateTimeW) {
        let today = chrono::Local::today();
        self.day_range(today)
    }

    fn yesterday_range(&self) -> (DateTimeW, DateTimeW) {
        let today = chrono::Local::today();
        let yesterday = today - chrono::Duration::days(1); // so proud
        self.day_range(yesterday)
    }

    fn last_week_range(&self) -> (DateTimeW, DateTimeW) {
        let today = chrono::Local::today();
        let weekday = today.weekday();
        let this_week_monday = today - Duration::days(weekday.num_days_from_monday() as i64);
        let last_week_monday = this_week_monday - Duration::days(7);
        let last_week_sunday = this_week_monday - Duration::days(1);
        self.days_range(last_week_monday, last_week_sunday)
    }
}

impl ChronoClock {
    fn day_range(&self, day: Date<Local>) -> (DateTimeW, DateTimeW) {
        self.days_range(day, day)
    }

    fn days_range(&self, day_start: Date<Local>, day_end: Date<Local>) -> (DateTimeW, DateTimeW) {
        (
            day_start.and_hms(0, 0, 0).into(),
            day_end.and_hms(23, 59, 59).into(),
        )
    }
}
