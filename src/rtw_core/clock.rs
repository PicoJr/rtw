use crate::rtw_core::datetimew::DateTimeW;

/// Time (absolute or relative)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Time {
    /// Now, can be converted to `DateTimeW` using `Clock.date_time`
    Now,
    DateTime(DateTimeW),
}

/// Clock Abstraction
pub trait Clock {
    /// Get current local time
    fn get_time(&self) -> DateTimeW;
    /// Convert a `Time` to absolute time
    ///
    /// `clock.date_time(Time::Now)` equals approximately clock.get_time();
    fn date_time(&self, time: Time) -> DateTimeW;

    /// Get time range for today
    ///
    /// today: 00:00:00 - 23:59:59
    fn today_range(&self) -> (DateTimeW, DateTimeW);

    /// Get time range for yesterday
    ///
    /// yesterday: 00:00:00 - 23:59:59
    fn yesterday_range(&self) -> (DateTimeW, DateTimeW);

    /// Get time range for last week
    ///
    /// last week (ISO 8601, week start on monday)
    ///
    /// last week: monday: 00:00:00 - sunday: 23:59:59
    fn last_week_range(&self) -> (DateTimeW, DateTimeW);

    /// Get time range for this week
    ///
    /// this week (ISO 8601, week start on monday)
    ///
    /// this week: monday: 00:00:00 - sunday: 23:59:59
    fn this_week_range(&self) -> (DateTimeW, DateTimeW);
}
