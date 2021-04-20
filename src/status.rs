use crate::rtw_core::clock::Clock;
use crate::rtw_core::service::ActivityService;
use crate::rtw_core::storage::Storage;
use crate::service::Service;
use chrono::Duration;
use chrono_humanize::HumanTime;
use itertools::Itertools;

pub(crate) type FormatString = String;

pub(crate) fn format_status<S, Cl>(
    format_string: Option<FormatString>,
    service: &Service<S>,
    clock: &Cl,
) -> anyhow::Result<String>
where
    S: Storage,
    Cl: Clock,
{
    let format_string = format_string.unwrap_or_else(|| String::from("{ongoing}"));
    let now = clock.get_time();
    let status = service
        .get_ongoing_activities()?
        .iter()
        .map(|(id, ongoing)| {
            let started: Duration = (ongoing.start_time - now).into();
            format_string
                .replace("{id}", &format!("{}", id))
                .replace("{ongoing}", &ongoing.get_title())
                .replace("{start}", &format!("{}", ongoing.start_time))
                .replace("{human_duration}", &format!("{}", HumanTime::from(started)))
                .replace("{duration}", &format!("{}", now - ongoing.start_time))
        })
        .join(" ");
    Ok(status)
}
