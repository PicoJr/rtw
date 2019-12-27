use crate::cli_helper;
use clap::ArgMatches;
use rtw::{ActiveActivity, Activity, ActivityService, Clock};

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
        let (start_time, tags) = cli_helper::ActivityCli::parse_start_args(sub_m)?;
        let abs_start_time = self.clock.abs_time(start_time);
        let started = self
            .service
            .start_activity(ActiveActivity::new(abs_start_time, tags))?;
        println!("Tracking {}", started.get_title());
        println!("Started  {}", started.get_start_time());
        Ok(())
    }

    fn run_stop(&mut self, sub_m: &ArgMatches) -> anyhow::Result<()> {
        let stop_time = cli_helper::ActivityCli::parse_stop_args(sub_m)?;
        let abs_stop_time = self.clock.abs_time(stop_time);
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
        let (range_start, range_end) =
            cli_helper::ActivityCli::parse_summary_args(sub_m, &self.clock)?;
        let activities = self.service.get_activities_within(range_start, range_end)?;
        if activities.is_empty() {
            println!("No filtered data found.");
        } else {
            let mut sorted: Vec<Activity> = activities;
            sorted.sort();
            for finished in sorted {
                let mut truncated_title = format!("{}", finished.get_title());
                truncated_title.truncate(12);
                println!(
                    "{:<12} {} {} {}",
                    truncated_title,
                    finished.get_start_time(),
                    finished.get_stop_time(),
                    finished.get_duration()
                );
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

    pub fn run(&mut self, matches: ArgMatches) -> anyhow::Result<()> {
        match matches.subcommand() {
            ("start", Some(sub_m)) => self.run_start(sub_m),
            ("stop", Some(sub_m)) => self.run_stop(sub_m),
            ("summary", Some(sub_m)) => self.run_summary(sub_m),
            // default case: display current activity
            _ => self.display_current(),
        }
    }
}
