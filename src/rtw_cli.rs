//! Translate CLI args to calls to activity Service.
use crate::cli_helper;
use crate::ical_export::export_activities_to_ical;
use crate::rtw_config::RTWConfig;
use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::clock::Clock;
use crate::rtw_core::datetimew::DateTimeW;
use crate::rtw_core::service::ActivityService;
use crate::rtw_core::storage::Storage;
use crate::rtw_core::ActivityId;
use crate::rtw_core::Tags;
use crate::service::Service;
use crate::timeline::render_days;
use clap::ArgMatches;

type ActivityWithId = (ActivityId, Activity);

/// Describe the action to be made
///
/// see `run`
pub enum RTWAction {
    CancelCurrent,
    Start(DateTimeW, Tags),
    Track((DateTimeW, DateTimeW), Tags),
    Stop(DateTimeW),
    Summary((DateTimeW, DateTimeW), bool),
    DumpICal((DateTimeW, DateTimeW)),
    Continue,
    Delete(ActivityId),
    DisplayCurrent,
    Timeline((DateTimeW, DateTimeW)),
    Completion(clap::Shell),
}

pub enum RTWMutation {
    Start(OngoingActivity),
    Track(Activity),
    Stop(DateTimeW),
    Delete(ActivityId),
    CancelCurrent,
    Pure,
}

/// Translate CLI args to actions (side-effect free)
///
/// It may fetch data from underlying activity storage but it should not write anything.
pub fn run<Cl>(matches: &ArgMatches, clock: &Cl) -> anyhow::Result<RTWAction>
where
    Cl: Clock,
{
    match matches.subcommand() {
        ("start", Some(sub_m)) => {
            let (start_time, tags) = cli_helper::parse_start_args(sub_m, clock)?;
            let abs_start_time = clock.date_time(start_time);
            Ok(RTWAction::Start(abs_start_time, tags))
        }
        ("stop", Some(sub_m)) => {
            let stop_time = cli_helper::parse_stop_args(sub_m, clock)?;
            let abs_stop_time = clock.date_time(stop_time);
            Ok(RTWAction::Stop(abs_stop_time))
        }
        ("summary", Some(sub_m)) => {
            let ((range_start, range_end), display_id) =
                cli_helper::parse_summary_args(sub_m, clock)?;
            Ok(RTWAction::Summary((range_start, range_end), display_id))
        }
        ("timeline", Some(sub_m)) => {
            let ((range_start, range_end), _display_id) =
                cli_helper::parse_timeline_args(sub_m, clock)?;
            Ok(RTWAction::Timeline((range_start, range_end)))
        }
        ("continue", Some(_sub_m)) => Ok(RTWAction::Continue),
        ("delete", Some(sub_m)) => {
            let id = cli_helper::parse_delete_args(sub_m)?;
            Ok(RTWAction::Delete(id))
        }
        ("track", Some(sub_m)) => {
            let (start_time, stop_time, tags) = cli_helper::parse_track_args(sub_m, clock)?;
            let start_time = clock.date_time(start_time);
            let stop_time = clock.date_time(stop_time);
            Ok(RTWAction::Track((start_time, stop_time), tags))
        }
        ("day", Some(_sub_m)) => {
            let (range_start, range_end) = clock.today_range();
            Ok(RTWAction::Timeline((range_start, range_end)))
        }
        ("week", Some(_sub_m)) => {
            let (range_start, range_end) = clock.this_week_range();
            Ok(RTWAction::Timeline((range_start, range_end)))
        }
        ("cancel", Some(_sub_m)) => Ok(RTWAction::CancelCurrent),
        ("dump", Some(sub_m)) => {
            let ((range_start, range_end), _display_id) =
                cli_helper::parse_summary_args(sub_m, clock)?;
            Ok(RTWAction::DumpICal((range_start, range_end)))
        }
        ("completion", Some(sub_m)) => {
            let shell = cli_helper::parse_completion_args(sub_m)?;
            Ok(RTWAction::Completion(shell))
        }
        // default case: display current activity
        _ => Ok(RTWAction::DisplayCurrent),
    }
}

/// Dry run (side effect-free)
pub fn dry_run_action<S, Cl>(
    action: RTWAction,
    service: &Service<S>,
    clock: &Cl,
    config: &RTWConfig,
) -> anyhow::Result<RTWMutation>
where
    S: Storage,
    Cl: Clock,
{
    match action {
        RTWAction::Start(start_time, tags) => {
            let started = OngoingActivity::new(start_time, tags);
            println!("Tracking {}", started.get_title());
            println!("Started  {}", started.get_start_time());
            Ok(RTWMutation::Start(started))
        }
        RTWAction::Track((start_time, stop_time), tags) => {
            let tracked = OngoingActivity::new(start_time, tags).into_activity(stop_time)?;
            println!("Recorded {}", tracked.get_title());
            println!("Started {:>20}", tracked.get_start_time());
            println!("Ended   {:>20}", tracked.get_stop_time());
            println!("Total   {:>20}", tracked.get_duration());
            Ok(RTWMutation::Track(tracked))
        }
        RTWAction::Stop(stop_time) => {
            let stopped_maybe = service.get_current_activity()?;
            match stopped_maybe {
                Some(stopped) => {
                    println!("Recorded {}", stopped.get_title());
                    println!("Started {:>20}", stopped.get_start_time());
                    println!("Ended   {:>20}", stop_time);
                    println!("Total   {:>20}", stop_time - stopped.get_start_time());
                    Ok(RTWMutation::Stop(stop_time))
                }
                None => {
                    println!("There is no active time tracking.");
                    Ok(RTWMutation::Pure)
                }
            }
        }
        RTWAction::Summary((range_start, range_end), display_id) => {
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
            Ok(RTWMutation::Pure)
        }
        RTWAction::Continue => {
            let activities = service.get_finished_activities()?;
            let last_activity_maybe = activities.last();
            match last_activity_maybe {
                None => {
                    println!("No activity to continue from.");
                    Ok(RTWMutation::Pure)
                }
                Some((_id, finished)) => {
                    println!("Tracking {}", finished.get_title());
                    let new_current = OngoingActivity::new(clock.get_time(), finished.get_tags());
                    Ok(RTWMutation::Start(new_current))
                }
            }
        }
        RTWAction::Delete(activity_id) => {
            let deleted = service.filter_activities(|(i, _)| *i == activity_id)?;
            let deleted_maybe = deleted.first();
            match deleted_maybe {
                None => {
                    println!("No activity found for id {}.", activity_id);
                    Ok(RTWMutation::Pure)
                }
                Some((deleted_id, deleted)) => {
                    println!("Deleted {}", deleted.get_title());
                    println!("Started {:>20}", deleted.get_start_time());
                    println!("Ended   {:>20}", deleted.get_stop_time());
                    println!("Total   {:>20}", deleted.get_duration());
                    Ok(RTWMutation::Delete(*deleted_id))
                }
            }
        }
        RTWAction::DisplayCurrent => {
            let activity_maybe = service.get_current_activity()?;
            match activity_maybe {
                Some(current) => {
                    println!("Tracking {}", current.get_title());
                    println!("Total    {}", clock.get_time() - current.get_start_time());
                }
                None => println!("There is no active time tracking."),
            }
            Ok(RTWMutation::Pure)
        }
        RTWAction::Timeline((range_start, range_end)) => {
            let activities = service.get_finished_activities()?;
            let activities: Vec<ActivityWithId> = activities
                .iter()
                .filter(|(_i, a)| {
                    range_start <= a.get_start_time() && a.get_start_time() <= range_end
                })
                .cloned()
                .collect();
            let rendered = render_days(activities.as_slice(), &config.timeline_colors)?;
            for line in rendered {
                println!("{}", line);
            }
            Ok(RTWMutation::Pure)
        }
        RTWAction::CancelCurrent => {
            let cancelled_maybe = service.get_current_activity()?;
            match cancelled_maybe {
                Some(cancelled) => {
                    println!("Cancelled {}", cancelled.get_title());
                    println!("Started   {:>20}", cancelled.get_start_time());
                    println!(
                        "Total     {:>20}",
                        clock.get_time() - cancelled.get_start_time()
                    );
                    Ok(RTWMutation::CancelCurrent)
                }
                None => {
                    println!("Nothing to cancel: there is no active time tracking.");
                    Ok(RTWMutation::Pure)
                }
            }
        }
        RTWAction::DumpICal((range_start, range_end)) => {
            let activities = service.get_finished_activities()?;
            let activities: Vec<Activity> = activities
                .iter()
                .map(|(_i, a)| a)
                .filter(|a| range_start <= a.get_start_time() && a.get_start_time() <= range_end)
                .cloned()
                .collect();
            let calendar = export_activities_to_ical(activities.as_slice());
            println!("{}", calendar);
            Ok(RTWMutation::Pure)
        }
        RTWAction::Completion(shell) => {
            let mut app = cli_helper::get_app();
            app.gen_completions_to(crate_name!(), shell, &mut std::io::stdout());
            Ok(RTWMutation::Pure)
        }
    }
}

/// Side effect
pub fn run_mutation<S>(
    action: RTWMutation,
    service: &mut Service<S>,
    config: &RTWConfig,
) -> anyhow::Result<()>
where
    S: Storage,
{
    match action {
        RTWMutation::Start(activity) => {
            let _started = service.start_activity(activity, config.deny_overlapping)?;
            Ok(())
        }
        RTWMutation::Track(activity) => {
            let _tracked = service.track_activity(activity, config.deny_overlapping)?;
            Ok(())
        }
        RTWMutation::Stop(stop_time) => {
            let _stopped = service.stop_current_activity(stop_time, config.deny_overlapping)?;
            Ok(())
        }
        RTWMutation::Delete(activity_id) => {
            let _deleted = service.delete_activity(activity_id)?;
            Ok(())
        }
        RTWMutation::CancelCurrent => {
            let _cancelled = service.cancel_current_activity()?;
            Ok(())
        }
        RTWMutation::Pure => {
            // pure nothing to do
            Ok(())
        }
    }
}
