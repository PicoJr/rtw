use anyhow::anyhow;
use chrono::{Local, TimeZone};
use regex::Regex;
use rtw::Time;
use rtw::DATETIME_FMT;
use rtw::RELATIVE_TIME_REGEX;
use std::str::FromStr;

pub struct TimeTools {}

impl TimeTools {
    pub fn is_time(s: &str) -> bool {
        let is_abs_time = TimeTools::is_abs_time(s);
        let is_relative_time = TimeTools::is_relative_time(s);
        is_abs_time || is_relative_time
    }

    fn is_abs_time(s: &str) -> bool {
        Local.datetime_from_str(s, DATETIME_FMT).is_ok()
    }

    fn is_relative_time(s: &str) -> bool {
        let re = Regex::new(RELATIVE_TIME_REGEX).unwrap();
        re.is_match(s)
    }

    fn abs_time_from_str(s: &str) -> anyhow::Result<Time> {
        match Local.datetime_from_str(s, DATETIME_FMT) {
            Ok(d) => Ok(Time::Abs(d.into())),
            Err(_) => Err(anyhow!("unrecognized time format")),
        }
    }

    fn relative_time_from_str(s: &str) -> anyhow::Result<Time> {
        let re = Regex::new(RELATIVE_TIME_REGEX).unwrap();
        let cap = re.captures(s);
        let minutes = cap
            .and_then(|c| c.get(1))
            .map(|s| s.as_str())
            .ok_or_else(|| anyhow!("invalid relative time: {}", s))?;
        usize::from_str(minutes)
            .and_then(|minutes| Ok(Time::MinutesAgo(minutes)))
            .or_else(|_| Err(anyhow!("invalid relative time: {}", s)))
    }

    pub fn time_from_str(s: &str) -> anyhow::Result<Time> {
        TimeTools::abs_time_from_str(s).or_else(|_| TimeTools::relative_time_from_str(s))
    }
}

#[cfg(test)]
mod test {
    use crate::time_tools::TimeTools;
    use rtw::Time;

    #[test]
    fn time_from_str() {
        let time = TimeTools::time_from_str("4m");
        assert_eq!(time.unwrap(), Time::MinutesAgo(4));
        let time = TimeTools::time_from_str("42m");
        assert_eq!(time.unwrap(), Time::MinutesAgo(42));
    }

    #[test]
    fn test_relative_time_from_str() {
        let time = TimeTools::relative_time_from_str("4m");
        assert_eq!(time.unwrap(), Time::MinutesAgo(4));
        let time = TimeTools::relative_time_from_str("42m");
        assert_eq!(time.unwrap(), Time::MinutesAgo(42));
    }

    #[test]
    fn test_is_relative_time() {
        // match
        assert!(TimeTools::is_relative_time("54m"));
        assert!(TimeTools::is_relative_time("32m"));
        assert!(TimeTools::is_relative_time("32min"));

        // no match
        assert!(!TimeTools::is_relative_time("m"));
        assert!(!TimeTools::is_relative_time("32 min"));
    }
}
