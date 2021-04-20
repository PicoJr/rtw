//! Translate CLI args to calls to activity Service.
use crate::cli_helper;
use crate::ical_export::export_activities_to_ical;
use crate::rtw_cli::OptionalOrAmbiguousOrNotFound::Optional;
use crate::rtw_config::RtwConfig;
use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::clock::Clock;
use crate::rtw_core::datetimew::DateTimeW;
use crate::rtw_core::durationw::DurationW;
use crate::rtw_core::service::ActivityService;
use crate::rtw_core::storage::Storage;
use crate::rtw_core::ActivityId;
use crate::rtw_core::{Description, Tags};
use crate::service::Service;
use crate::status::{format_status, FormatString};
use crate::timeline::render_days;
use clap::ArgMatches;
use itertools::Itertools;

type ActivityWithId = (ActivityId, Activity);

/// Describe the action to be made
///
/// see `run`
pub enum RtwAction {
    Cancel(Option<ActivityId>),
    Start(DateTimeW, Tags, Option<Description>),
    Track((DateTimeW, DateTimeW), Tags, Option<Description>),
    Stop(DateTimeW, Option<ActivityId>),
    Summary((DateTimeW, DateTimeW), bool, bool, bool),
    DumpICal((DateTimeW, DateTimeW)),
    Continue(Option<ActivityId>),
    Delete(ActivityId),
    DisplayCurrent,
    Timeline((DateTimeW, DateTimeW)),
    Completion(clap::Shell),
    Status(Option<FormatString>),
}

pub enum RtwMutation {
    Start(OngoingActivity),
    Track(Activity),
    Stop(DateTimeW, ActivityId),
    Delete(ActivityId),
    Cancel(ActivityId),
    Pure,
}

enum OptionalOrAmbiguousOrNotFound {
    Optional(Option<(ActivityId, OngoingActivity)>),
    Ambiguous,
    NotFound(ActivityId),
}

fn merge_same_tags(activities: &[ActivityWithId]) -> Vec<(ActivityId, Activity, DurationW, usize)> {
    let uniques: Vec<ActivityWithId> = activities
        .iter()
        .cloned()
        .unique_by(|(_i, activity)| activity.get_title())
        .collect();
    uniques
        .iter()
        .cloned()
        .map(|(i, activity)| {
            let same_tag = activities
                .iter()
                .filter(|(_i, other)| activity.get_title() == other.get_title());
            let durations: Vec<DurationW> = same_tag.map(|(_i, a)| a.get_duration()).collect();
            let segments = durations.len();
            let duration = durations.into_iter().sum();
            (i, activity, duration, segments)
        })
        .collect()
}

fn get_ongoing_activity<S: Storage>(
    id_maybe: Option<ActivityId>,
    service: &Service<S>,
) -> anyhow::Result<OptionalOrAmbiguousOrNotFound> {
    match id_maybe {
        None => match service.get_ongoing_activities()?.as_slice() {
            [] => Ok(OptionalOrAmbiguousOrNotFound::Optional(None)),
            [(cancelled_id, cancelled)] => Ok(OptionalOrAmbiguousOrNotFound::Optional(Some((
                *cancelled_id,
                cancelled.clone(),
            )))),
            _ => Ok(OptionalOrAmbiguousOrNotFound::Ambiguous),
        },
        Some(cancelled_id) => match service.get_ongoing_activity(cancelled_id)? {
            None => Ok(OptionalOrAmbiguousOrNotFound::NotFound(cancelled_id)),
            Some(cancelled) => Ok(OptionalOrAmbiguousOrNotFound::Optional(Some((
                cancelled_id,
                cancelled,
            )))),
        },
    }
}

/// Translate CLI args to actions (side-effect free)
///
/// It may fetch data from underlying activity storage but it should not write anything.
pub fn run<Cl>(matches: &ArgMatches, clock: &Cl) -> anyhow::Result<RtwAction>
where
    Cl: Clock,
{
    match matches.subcommand() {
        ("start", Some(sub_m)) => {
            let (start_time, tags, description) = cli_helper::parse_start_args(sub_m, clock)?;
            let abs_start_time = clock.date_time(start_time);
            Ok(RtwAction::Start(abs_start_time, tags, description))
        }
        ("stop", Some(sub_m)) => {
            let (stop_time, stopped_id_maybe) = cli_helper::parse_stop_args(sub_m, clock)?;
            let abs_stop_time = clock.date_time(stop_time);
            Ok(RtwAction::Stop(abs_stop_time, stopped_id_maybe))
        }
        ("summary", Some(sub_m)) => {
            let ((range_start, range_end), display_id, display_description, report) =
                cli_helper::parse_summary_args(sub_m, clock)?;
            Ok(RtwAction::Summary(
                (range_start, range_end),
                display_id,
                display_description,
                report,
            ))
        }
        ("timeline", Some(sub_m)) => {
            let ((range_start, range_end), _display_id) =
                cli_helper::parse_timeline_args(sub_m, clock)?;
            Ok(RtwAction::Timeline((range_start, range_end)))
        }
        ("continue", Some(sub_m)) => {
            let continue_id_maybe = cli_helper::parse_continue_args(sub_m)?;
            Ok(RtwAction::Continue(continue_id_maybe))
        }
        ("delete", Some(sub_m)) => {
            let id = cli_helper::parse_delete_args(sub_m)?;
            Ok(RtwAction::Delete(id))
        }
        ("track", Some(sub_m)) => {
            let (start_time, stop_time, tags, description) =
                cli_helper::parse_track_args(sub_m, clock)?;
            let start_time = clock.date_time(start_time);
            let stop_time = clock.date_time(stop_time);
            Ok(RtwAction::Track((start_time, stop_time), tags, description))
        }
        ("day", Some(_sub_m)) => {
            let (range_start, range_end) = clock.today_range();
            Ok(RtwAction::Timeline((range_start, range_end)))
        }
        ("week", Some(_sub_m)) => {
            let (range_start, range_end) = clock.this_week_range();
            Ok(RtwAction::Timeline((range_start, range_end)))
        }
        ("cancel", Some(sub_m)) => {
            let cancelled_id_maybe = cli_helper::parse_cancel_args(sub_m)?;
            Ok(RtwAction::Cancel(cancelled_id_maybe))
        }
        ("dump", Some(sub_m)) => {
            let ((range_start, range_end), _display_id, _description, _report) =
                cli_helper::parse_summary_args(sub_m, clock)?;
            Ok(RtwAction::DumpICal((range_start, range_end)))
        }
        ("completion", Some(sub_m)) => {
            let shell = cli_helper::parse_completion_args(sub_m)?;
            Ok(RtwAction::Completion(shell))
        }
        ("status", Some(sub_m)) => {
            let format = cli_helper::parse_status_args(sub_m);
            Ok(RtwAction::Status(format))
        }
        // default case: display current activity
        _ => Ok(RtwAction::DisplayCurrent),
    }
}

/// Dry run (side effect-free)
pub fn dry_run_action<S, Cl>(
    action: RtwAction,
    service: &Service<S>,
    clock: &Cl,
    config: &RtwConfig,
) -> anyhow::Result<RtwMutation>
where
    S: Storage,
    Cl: Clock,
{
    match action {
        RtwAction::Start(start_time, tags, description) => {
            let started = OngoingActivity::new(start_time, tags, description);
            println!("Tracking {}", started.get_title());
            println!("Started  {}", started.get_start_time());
            Ok(RtwMutation::Start(started))
        }
        RtwAction::Track((start_time, stop_time), tags, description) => {
            let tracked =
                OngoingActivity::new(start_time, tags, description).into_activity(stop_time)?;
            println!("Recorded {}", tracked.get_title());
            println!("Started {:>20}", tracked.get_start_time());
            println!("Ended   {:>20}", tracked.get_stop_time());
            println!("Total   {:>20}", tracked.get_duration());
            Ok(RtwMutation::Track(tracked))
        }
        RtwAction::Stop(stop_time, activity_id) => {
            match get_ongoing_activity(activity_id, &service)? {
                Optional(None) => {
                    println!("There is no active time tracking.");
                    Ok(RtwMutation::Pure)
                }
                Optional(Some((stopped_id, stopped))) => {
                    println!("Recorded {}", stopped.get_title());
                    println!("Started {:>20}", stopped.get_start_time());
                    println!("Ended   {:>20}", stop_time);
                    println!("Total   {:>20}", stop_time - stopped.get_start_time());
                    Ok(RtwMutation::Stop(stop_time, stopped_id))
                }
                OptionalOrAmbiguousOrNotFound::Ambiguous => {
                    println!("Multiple ongoing activities, please provide an id.");
                    Ok(RtwMutation::Pure)
                }
                OptionalOrAmbiguousOrNotFound::NotFound(stopped_id) => {
                    println!("No ongoing activity with id {}.", stopped_id);
                    Ok(RtwMutation::Pure)
                }
            }
        }
        RtwAction::Summary((range_start, range_end), display_id, display_description, report) => {
            let activities = service.get_finished_activities()?;
            let activities: Vec<(ActivityId, Activity)> = activities
                .iter()
                .filter(|(_i, a)| {
                    range_start <= a.get_start_time() && a.get_start_time() <= range_end
                })
                .cloned()
                .collect();
            let longest_title = activities
                .iter()
                .map(|(_id, a)| a.get_title().len())
                .max()
                .unwrap_or_default();
            if activities.is_empty() {
                println!("No filtered data found.");
            } else if report {
                let activities_report = merge_same_tags(activities.as_slice());
                for (_id, finished, duration, segments) in activities_report {
                    let singular_or_plural = if segments <= 1 {
                        String::from("segment")
                    } else {
                        // segments > 1
                        String::from("segments")
                    };
                    let output = format!(
                        "{:width$} {} ({} {})",
                        finished.get_title(),
                        duration,
                        segments,
                        singular_or_plural,
                        width = longest_title
                    );
                    println!("{}", output)
                }
            } else {
                for (id, finished) in activities {
                    let output = format!(
                        "{:width$} {} {} {}",
                        finished.get_title(),
                        finished.get_start_time(),
                        finished.get_stop_time(),
                        finished.get_duration(),
                        width = longest_title
                    );
                    let output = if display_id {
                        format!("{:>1} {}", id, output)
                    } else {
                        output
                    };
                    let output = match (display_description, finished.get_description()) {
                        (false, _) => output,
                        (true, None) => output,
                        (true, Some(description)) => format!("{}\n{}", output, description),
                    };
                    println!("{}", output)
                }
            }
            Ok(RtwMutation::Pure)
        }
        RtwAction::Continue(activity_id) => {
            let activities = service.get_finished_activities()?;
            let activity_id = activity_id.unwrap_or(0); // id 0 == last finished activity
            let continued_maybe = activities.iter().find(|(id, _a)| *id == activity_id);
            match continued_maybe {
                None => {
                    println!("No activity to continue from.");
                    Ok(RtwMutation::Pure)
                }
                Some((_id, finished)) => {
                    println!("Tracking {}", finished.get_title());
                    let new_current = OngoingActivity::new(
                        clock.get_time(),
                        finished.get_tags(),
                        finished.get_description(),
                    );
                    Ok(RtwMutation::Start(new_current))
                }
            }
        }
        RtwAction::Delete(activity_id) => {
            let deleted = service.filter_activities(|(i, _)| *i == activity_id)?;
            let deleted_maybe = deleted.first();
            match deleted_maybe {
                None => {
                    println!("No activity found for id {}.", activity_id);
                    Ok(RtwMutation::Pure)
                }
                Some((deleted_id, deleted)) => {
                    println!("Deleted {}", deleted.get_title());
                    println!("Started {:>20}", deleted.get_start_time());
                    println!("Ended   {:>20}", deleted.get_stop_time());
                    println!("Total   {:>20}", deleted.get_duration());
                    Ok(RtwMutation::Delete(*deleted_id))
                }
            }
        }
        RtwAction::DisplayCurrent => {
            let ongoing_activities = service.get_ongoing_activities()?;
            if ongoing_activities.is_empty() {
                println!("There is no active time tracking.");
            } else {
                for (id, ongoing_activity) in ongoing_activities {
                    println!("Tracking {}", ongoing_activity.get_title());
                    println!(
                        "Total    {}",
                        clock.get_time() - ongoing_activity.get_start_time()
                    );
                    println!("Id       {}", id);
                }
            }
            Ok(RtwMutation::Pure)
        }
        RtwAction::Timeline((range_start, range_end)) => {
            let activities = service.get_finished_activities()?;
            let activities: Vec<ActivityWithId> = activities
                .iter()
                .filter(|(_i, a)| {
                    range_start <= a.get_start_time() && a.get_start_time() <= range_end
                })
                .cloned()
                .collect();
            let now = clock.get_time();
            let ongoing_activities = service.get_ongoing_activities()?;
            let ongoing_activities: Vec<ActivityWithId> = ongoing_activities
                .iter()
                .filter(|(_i, a)| {
                    range_start <= a.get_start_time() && a.get_start_time() <= range_end
                })
                .filter_map(|(i, a)| match a.clone().into_activity(now) {
                    Ok(a) => Some((*i, a)),
                    _ => None,
                })
                .collect();
            let timeline_activities: Vec<ActivityWithId> = activities
                .iter()
                .cloned()
                .chain(ongoing_activities.iter().cloned())
                .collect();
            let rendered = render_days(timeline_activities.as_slice(), &config.timeline_colors)?;
            for line in rendered {
                println!("{}", line);
            }
            Ok(RtwMutation::Pure)
        }
        RtwAction::Cancel(id_maybe) => match get_ongoing_activity(id_maybe, service)? {
            Optional(None) => {
                println!("Nothing to cancel: there is no active time tracking.");
                Ok(RtwMutation::Pure)
            }
            Optional(Some((cancelled_id, cancelled))) => {
                println!("Cancelled {}", cancelled.get_title());
                println!("Started   {:>20}", cancelled.get_start_time());
                println!(
                    "Total     {:>20}",
                    clock.get_time() - cancelled.get_start_time()
                );
                Ok(RtwMutation::Cancel(cancelled_id))
            }
            OptionalOrAmbiguousOrNotFound::Ambiguous => {
                println!("Multiple ongoing activities, please provide an id.");
                Ok(RtwMutation::Pure)
            }
            OptionalOrAmbiguousOrNotFound::NotFound(cancelled_id) => {
                println!("No ongoing activity with id {}.", cancelled_id);
                Ok(RtwMutation::Pure)
            }
        },
        RtwAction::DumpICal((range_start, range_end)) => {
            let activities = service.get_finished_activities()?;
            let activities: Vec<Activity> = activities
                .iter()
                .map(|(_i, a)| a)
                .filter(|a| range_start <= a.get_start_time() && a.get_start_time() <= range_end)
                .cloned()
                .collect();
            let calendar = export_activities_to_ical(activities.as_slice());
            println!("{}", calendar);
            Ok(RtwMutation::Pure)
        }
        RtwAction::Completion(shell) => {
            let mut app = cli_helper::get_app();
            app.gen_completions_to(crate_name!(), shell, &mut std::io::stdout());
            Ok(RtwMutation::Pure)
        }
        RtwAction::Status(format_maybe) => {
            let status = format_status(format_maybe, service, clock)?;
            println!("{}", status);
            Ok(RtwMutation::Pure)
        }
    }
}

/// Side effect
pub fn run_mutation<S>(
    action: RtwMutation,
    service: &mut Service<S>,
    config: &RtwConfig,
) -> anyhow::Result<()>
where
    S: Storage,
{
    match action {
        RtwMutation::Start(activity) => {
            let _started = service.start_activity(activity, config.deny_overlapping)?;
            Ok(())
        }
        RtwMutation::Track(activity) => {
            let _tracked = service.track_activity(activity, config.deny_overlapping)?;
            Ok(())
        }
        RtwMutation::Stop(stop_time, activity_id) => {
            let _stopped =
                service.stop_ongoing_activity(stop_time, activity_id, config.deny_overlapping)?;
            Ok(())
        }
        RtwMutation::Delete(activity_id) => {
            let _deleted = service.delete_activity(activity_id)?;
            Ok(())
        }
        RtwMutation::Cancel(activity_id) => {
            let _cancelled = service.cancel_ongoing_activity(activity_id)?;
            Ok(())
        }
        RtwMutation::Pure => {
            // pure nothing to do
            Ok(())
        }
    }
}
