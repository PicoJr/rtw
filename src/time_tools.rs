//! Time parsing utils.
use crate::rtw_core::clock::{Clock, Time};
use anyhow::anyhow;
use chrono::Local;
use htp::parse;

pub struct TimeTools {}

impl TimeTools {
    pub fn is_time(s: &str) -> bool {
        parse(s, Local::now()).is_ok()
    }

    pub fn time_from_str(s: &str, clock: &dyn Clock) -> anyhow::Result<Time> {
        match parse(s, clock.get_time().into()) {
            Ok(dt) => Ok(Time::DateTime(dt.into())),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}
