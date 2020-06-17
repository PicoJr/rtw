use crate::rtw_core::activity::Activity;
use crate::rtw_core::datetimew::DateTimeW;
use chrono::{DateTime, Local};
use icalendar::Calendar;
use icalendar::CalendarDateTime;
use icalendar::Component;
use icalendar::Event;

impl From<DateTimeW> for CalendarDateTime {
    fn from(d: DateTimeW) -> Self {
        let local: DateTime<Local> = d.into();
        local.naive_utc().into()
    }
}

impl From<Activity> for Event {
    fn from(a: Activity) -> Self {
        let start_time = a.get_start_time();
        let stop_time = a.get_stop_time();
        let title = a.get_title();
        Event::new()
            .summary(title.as_str())
            .starts(start_time)
            .ends(stop_time)
            .done()
    }
}

pub(crate) fn export_activities_to_ical(activities: &[Activity]) -> Calendar {
    let mut calendar = Calendar::new();
    for activity in activities {
        let event: Event = activity.clone().into();
        calendar.push(event);
    }
    calendar
}
