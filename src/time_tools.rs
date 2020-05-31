use crate::rtw_core::clock::{Clock, Time};
use anyhow::anyhow;
use chrono::Local;
use chrono_english::{parse_date_string, Dialect};

pub struct TimeTools {}

impl TimeTools {
    pub fn is_time(s: &str) -> bool {
        parse_date_string(s, Local::now(), Dialect::Uk).is_ok()
    }

    pub fn time_from_str(s: &str, clock: &dyn Clock) -> anyhow::Result<Time> {
        match parse_date_string(s, clock.get_time().into(), Dialect::Uk) {
            Ok(dt) => Ok(Time::DateTime(dt.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}
