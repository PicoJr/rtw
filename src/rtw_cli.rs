use crate::cli_helper;
use crate::timeline::render_days;
use clap::ArgMatches;
use rtw::{Activity, ActivityId, ActivityService, Clock, OngoingActivity};

pub struct RTW<C, S>
where
    C: Clock,
    S: ActivityService,
{
    clock: C,
    service: S,
}

impl<C, S> RTW<C, S>
where
    C: Clock,
    S: ActivityService,
{
    pub fn new(clock: C, service: S) -> Self {
        RTW { clock, service }
    }

    fn run_start(&mut self, sub_m: &ArgMatches) -> anyhow::Result<()> {
        let (start_time, tags) = cli_helper::ActivityCli::parse_start_args(sub_m, &self.clock)?;
        let abs_start_time = self.clock.date_time(start_time);
        let started = self
            .service
            .start_activity(OngoingActivity::new(abs_start_time, tags))?;
        println!("Tracking {}", started.get_title());
        println!("Started  {}", started.get_start_time());
        Ok(())
    }

    fn run_track(&mut self, sub_m: &ArgMatches) -> anyhow::Result<()> {
        let (start_time, stop_time, tags) =
            cli_helper::ActivityCli::parse_track_args(sub_m, &self.clock)?;
        let activity = OngoingActivity::new(self.clock.date_time(start_time), tags)
            .into_activity(self.clock.date_time(stop_time))?;
        let tracked = self.service.track_activity(activity)?;
        println!("Recorded {}", tracked.get_title());
        println!("Started {:>20}", tracked.get_start_time());
        println!("Ended   {:>20}", tracked.get_stop_time());
        println!("Total   {:>20}", tracked.get_duration());
        Ok(())
    }

    fn run_stop(&mut self, sub_m: &ArgMatches) -> anyhow::Result<()> {
        let stop_time = cli_helper::ActivityCli::parse_stop_args(sub_m, &self.clock)?;
        let abs_stop_time = self.clock.date_time(stop_time);
        let stopped_maybe = self.service.stop_current_activity(abs_stop_time)?;
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

    fn run_summary(&mut self, sub_m: &ArgMatches) -> anyhow::Result<()> {
        let ((range_start, range_end), display_id) =
            cli_helper::ActivityCli::parse_summary_args(sub_m, &self.clock)?;
        let activities: Vec<(ActivityId, Activity)> =
            self.service.filter_activities(|(_i, a)| {
                range_start <= a.get_start_time() && a.get_start_time() <= range_end
            })?;
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
                    output = format!("{:>2} {}", id, output);
                }
                println!("{}", output)
            }
        }
        Ok(())
    }

    fn run_continue(&mut self, _sub_m: &ArgMatches) -> anyhow::Result<()> {
        let activities: Vec<(ActivityId, Activity)> =
            self.service.filter_activities(|(_, _)| true)?;
        match activities.last() {
            None => println!("No activity to continue from."),
            Some((_id, finished)) => {
                self.service.start_activity(OngoingActivity::new(
                    self.clock.get_time(),
                    finished.get_tags(),
                ))?;
                self.display_current()?;
            }
        }
        Ok(())
    }

    fn run_delete(&mut self, sub_m: &ArgMatches) -> anyhow::Result<()> {
        let id = cli_helper::ActivityCli::parse_delete_args(sub_m)?;
        let deleted_maybe = self.service.delete_activity(id)?;
        match deleted_maybe {
            None => println!("No activity found for id {}.", id),
            Some(deleted) => {
                println!("Deleted {}", deleted.get_title());
                println!("Started {:>20}", deleted.get_start_time());
                println!("Ended   {:>20}", deleted.get_stop_time());
                println!("Total   {:>20}", deleted.get_duration());
            }
        }
        Ok(())
    }

    fn display_current(&mut self) -> anyhow::Result<()> {
        let current = self.service.get_current_activity()?;
        match current {
            Some(current) => {
                println!("Tracking {}", current.get_title());
                println!(
                    "Total    {}",
                    self.clock.get_time() - current.get_start_time()
                );
            }
            None => println!("There is no active time tracking."),
        }
        Ok(())
    }

    fn timeline_day(&mut self, _sub_m: &ArgMatches) -> anyhow::Result<()> {
        let (range_start, range_end) = self.clock.today_range();
        let activities: Vec<(ActivityId, Activity)> =
            self.service.filter_activities(|(_i, a)| {
                range_start <= a.get_start_time() && a.get_start_time() <= range_end
            })?;
        let rendered = render_days(activities.as_slice())?;
        for line in rendered {
            println!("{}", line);
        }
        Ok(())
    }

    fn timeline_week(&mut self, _sub_m: &ArgMatches) -> anyhow::Result<()> {
        let (range_start, range_end) = self.clock.this_week_range();
        let activities: Vec<(ActivityId, Activity)> =
            self.service.filter_activities(|(_i, a)| {
                range_start <= a.get_start_time() && a.get_start_time() <= range_end
            })?;
        let rendered = render_days(activities.as_slice())?;
        for line in rendered {
            println!("{}", line);
        }
        Ok(())
    }

    pub fn run(&mut self, matches: ArgMatches) -> anyhow::Result<()> {
        match matches.subcommand() {
            ("start", Some(sub_m)) => self.run_start(sub_m),
            ("stop", Some(sub_m)) => self.run_stop(sub_m),
            ("summary", Some(sub_m)) => self.run_summary(sub_m),
            ("continue", Some(sub_m)) => self.run_continue(sub_m),
            ("delete", Some(sub_m)) => self.run_delete(sub_m),
            ("track", Some(sub_m)) => self.run_track(sub_m),
            ("day", Some(sub_m)) => self.timeline_day(sub_m),
            ("week", Some(sub_m)) => self.timeline_week(sub_m),
            // default case: display current activity
            _ => self.display_current(),
        }
    }
}
