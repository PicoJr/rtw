use crate::rtw_core::clock::Clock;
use crate::rtw_core::service::ActivityService;
use crate::rtw_core::storage::Storage;
use crate::service::Service;
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
    let ongoing_activities = || -> anyhow::Result<String> {
        let ongoing_actvities = service.get_ongoing_activities()?;
        Ok(ongoing_actvities
            .iter()
            .map(|(_, ongoing)| ongoing.get_title())
            .join(","))
    };
    let status = format_string.replace("{ongoing}", &ongoing_activities()?);
    Ok(status)
}
