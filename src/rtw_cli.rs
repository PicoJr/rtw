//! Translate CLI args to calls to activity Service.
use crate::cli_helper;
use crate::rtw_config::RTWConfig;
use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::clock::Clock;
use crate::rtw_core::datetimew::DateTimeW;
use crate::rtw_core::service::ActivityService;
use crate::rtw_core::storage::Storage;
use crate::rtw_core::{ActivityId, Tags};
use crate::service::Service;
use crate::timeline::render_days;
use clap::ArgMatches;

type ActivityWithId = (ActivityId, Activity);

/// Describe the action (side-effect) to be made
///
/// see `run`
pub enum RTWAction {
    Cancel(Option<OngoingActivity>),
    Start(OngoingActivity),
    Track(Activity),
    Stop(DateTimeW),
    Summary(Vec<ActivityWithId>, bool),
    Continue(Option<ActivityWithId>),
    Delete(ActivityId),
    Display(Option<OngoingActivity>),
    Timeline(Vec<ActivityWithId>),
}

fn run_start(start_time: DateTimeW, tags: Tags) -> anyhow::Result<RTWAction> {
    Ok(RTWAction::Start(OngoingActivity::new(start_time, tags)))
}

fn run_track(start_time: DateTimeW, stop_time: DateTimeW, tags: Tags) -> anyhow::Result<RTWAction> {
    let activity = OngoingActivity::new(start_time, tags).into_activity(stop_time)?;
    Ok(RTWAction::Track(activity))
}

fn run_stop(stop_time: DateTimeW) -> anyhow::Result<RTWAction> {
    Ok(RTWAction::Stop(stop_time))
}

fn run_summary(
    range_start: DateTimeW,
    range_end: DateTimeW,
    display_id: bool,
    activities: &[ActivityWithId],
) -> anyhow::Result<RTWAction> {
    let activities: Vec<(ActivityId, Activity)> = activities
        .iter()
        .filter(|(_i, a)| range_start <= a.get_start_time() && a.get_start_time() <= range_end)
        .cloned()
        .collect();
    Ok(RTWAction::Summary(activities, display_id))
}

fn run_continue(activities: &[ActivityWithId]) -> anyhow::Result<RTWAction> {
    let last_activity = activities.last();
    Ok(RTWAction::Continue(last_activity.cloned()))
}

fn run_delete(id: ActivityId) -> anyhow::Result<RTWAction> {
    Ok(RTWAction::Delete(id))
}

fn display_current(current_maybe: Option<OngoingActivity>) -> anyhow::Result<RTWAction> {
    Ok(RTWAction::Display(current_maybe))
}

fn cancel_current(current_maybe: Option<OngoingActivity>) -> anyhow::Result<RTWAction> {
    Ok(RTWAction::Cancel(current_maybe))
}

fn run_timeline(
    range_start: DateTimeW,
    range_end: DateTimeW,
    _display_id: bool,
    activities: &[ActivityWithId],
) -> anyhow::Result<RTWAction> {
    let activities: Vec<ActivityWithId> = activities
        .iter()
        .filter(|(_i, a)| range_start <= a.get_start_time() && a.get_start_time() <= range_end)
        .cloned()
        .collect();
    Ok(RTWAction::Timeline(activities))
}

/// Translate CLI args to actions (mostly side-effect free)
///
/// It may fetch data from underlying activity storage but it should not write anything.
pub fn run<S, Cl>(
    matches: ArgMatches,
    service: &mut Service<S>,
    clock: &Cl,
) -> anyhow::Result<RTWAction>
where
    S: Storage,
    Cl: Clock,
{
    match matches.subcommand() {
        ("start", Some(sub_m)) => {
            let (start_time, tags) = cli_helper::parse_start_args(sub_m, clock)?;
            let abs_start_time = clock.date_time(start_time);
            run_start(abs_start_time, tags)
        }
        ("stop", Some(sub_m)) => {
            let stop_time = cli_helper::parse_stop_args(sub_m, clock)?;
            let abs_stop_time = clock.date_time(stop_time);
            run_stop(abs_stop_time)
        }
        ("summary", Some(sub_m)) => {
            let ((range_start, range_end), display_id) =
                cli_helper::parse_summary_args(sub_m, clock)?;
            let activities = service.get_finished_activities()?;
            run_summary(range_start, range_end, display_id, &activities)
        }
        ("timeline", Some(sub_m)) => {
            let ((range_start, range_end), display_id) =
                cli_helper::parse_timeline_args(sub_m, clock)?;
            let activities = service.get_finished_activities()?;
            run_timeline(range_start, range_end, display_id, &activities)
        }
        ("continue", Some(_sub_m)) => {
            let activities = service.get_finished_activities()?;
            run_continue(&activities)
        }
        ("delete", Some(sub_m)) => {
            let id = cli_helper::parse_delete_args(sub_m)?;
            run_delete(id)
        }
        ("track", Some(sub_m)) => {
            let (start_time, stop_time, tags) = cli_helper::parse_track_args(sub_m, clock)?;
            let start_time = clock.date_time(start_time);
            let stop_time = clock.date_time(stop_time);
            run_track(start_time, stop_time, tags)
        }
        ("day", Some(_sub_m)) => {
            let (range_start, range_end) = clock.today_range();
            let activities = service.get_finished_activities()?;
            run_timeline(range_start, range_end, false, &activities)
        }
        ("week", Some(_sub_m)) => {
            let (range_start, range_end) = clock.this_week_range();
            let activities = service.get_finished_activities()?;
            run_timeline(range_start, range_end, false, &activities)
        }
        ("cancel", Some(_sub_m)) => {
            let current = service.get_current_activity()?;
            cancel_current(current)
        }
        // default case: display current activity
        _ => {
            let current = service.get_current_activity()?;
            display_current(current)
        }
    }
}

/// Effectively perform the action (side effect).
pub fn run_action<S, Cl>(
    action: RTWAction,
    service: &mut Service<S>,
    clock: &Cl,
    config: &RTWConfig,
) -> anyhow::Result<()>
where
    S: Storage,
    Cl: Clock,
{
    match action {
        RTWAction::Start(activity) => {
            let started = service.start_activity(activity)?;
            println!("Tracking {}", started.get_title());
            println!("Started  {}", started.get_start_time());
            Ok(())
        }
        RTWAction::Track(activity) => {
            let tracked = service.track_activity(activity)?;
            println!("Recorded {}", tracked.get_title());
            println!("Started {:>20}", tracked.get_start_time());
            println!("Ended   {:>20}", tracked.get_stop_time());
            println!("Total   {:>20}", tracked.get_duration());
            Ok(())
        }
        RTWAction::Stop(stop_time) => {
            let stopped_maybe = service.stop_current_activity(stop_time)?;
            match stopped_maybe {
                Some(stopped) => {
                    println!("Recorded {}", stopped.get_title());
                    println!("Started {:>20}", stopped.get_start_time());
                    println!("Ended   {:>20}", stopped.get_stop_time());
                    println!("Total   {:>20}", stopped.get_duration());
                }
                None => println!("There is no active time tracking."),
            }
            Ok(())
        }
        RTWAction::Summary(activities, display_id) => {
            let longest_title = activities
                .iter()
                .map(|(_id, a)| a.get_title().len())
                .max()
                .unwrap_or_default();
            if activities.is_empty() {
                println!("No filtered data found.");
            } else {
                for (id, finished) in activities {
                    let mut output = format!(
                        "{:width$} {} {} {}",
                        finished.get_title(),
                        finished.get_start_time(),
                        finished.get_stop_time(),
                        finished.get_duration(),
                        width = longest_title
                    );
                    if display_id {
                        output = format!("{:>1} {}", id, output);
                    }
                    println!("{}", output)
                }
            }
            Ok(())
        }
        RTWAction::Continue(last_activity_maybe) => {
            match last_activity_maybe {
                None => println!("No activity to continue from."),
                Some((_id, finished)) => {
                    service.start_activity(OngoingActivity::new(
                        clock.get_time(),
                        finished.get_tags(),
                    ))?;
                    let current = service.get_current_activity()?;
                    if let Some(current) = current {
                        println!("Tracking {}", current.get_title());
                        println!("Total    {}", clock.get_time() - current.get_start_time());
                    }
                }
            }
            Ok(())
        }
        RTWAction::Delete(activity_id) => {
            let deleted_maybe = service.delete_activity(activity_id)?;
            match deleted_maybe {
                None => println!("No activity found for id {}.", activity_id),
                Some(deleted) => {
                    println!("Deleted {}", deleted.get_title());
                    println!("Started {:>20}", deleted.get_start_time());
                    println!("Ended   {:>20}", deleted.get_stop_time());
                    println!("Total   {:>20}", deleted.get_duration());
                }
            }
            Ok(())
        }
        RTWAction::Display(activity_maybe) => {
            match activity_maybe {
                Some(current) => {
                    println!("Tracking {}", current.get_title());
                    println!("Total    {}", clock.get_time() - current.get_start_time());
                }
                None => println!("There is no active time tracking."),
            }
            Ok(())
        }
        RTWAction::Timeline(activities) => {
            let rendered = render_days(activities.as_slice(), &config.timeline_colors)?;
            for line in rendered {
                println!("{}", line);
            }
            Ok(())
        }
        RTWAction::Cancel(_current_maybe) => {
            let cancelled_maybe = service.cancel_current_activity()?;
            match cancelled_maybe {
                Some(cancelled) => {
                    println!("Cancelled {}", cancelled.get_title());
                    println!("Started   {:>20}", cancelled.get_start_time());
                    println!(
                        "Total     {:>20}",
                        clock.get_time() - cancelled.get_start_time()
                    );
                }
                None => println!("Nothing to cancel: there is no active time tracking."),
            }
            Ok(())
        }
    }
}
