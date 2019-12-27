use clap::{App, Arg, ArgMatches, SubCommand};

use crate::time_tools::TimeTools;
use rtw::{AbsTime, Clock, Tags, Time};

pub struct ActivityCli {}

impl ActivityCli {
    pub fn get_app(&self) -> App {
        App::new("RTW")
            .version("0.1.1")
            .author("PicoJr")
            .about("rust time tracking CLI")
            .arg(
                Arg::with_name("directory")
                    .short("d")
                    .long("dir")
                    .value_name("DIR")
                    .required(false)
                    .help("custom directory")
                    .takes_value(true),
            )
            .subcommand(
                SubCommand::with_name("start")
                    .about("Start new activity")
                    .arg(Arg::with_name("time").help("time"))
                    .arg(Arg::with_name("tags").multiple(true).help("tags")),
            )
            .subcommand(
                SubCommand::with_name("stop")
                    .about("Stop activity")
                    .arg(Arg::with_name("time").help("time")),
            )
            .subcommand(
                SubCommand::with_name("summary")
                    .about("Display finished activities")
                    .arg(
                        Arg::with_name("yesterday")
                            .long("yesterday")
                            .help("activities done yesterday"),
                    )
                    .arg(
                        Arg::with_name("lastweek")
                            .long("lastweek")
                            .help("activities done last week"),
                    ),
            )
            .subcommand(SubCommand::with_name("continue").about("continue a finished activity"))
    }

    pub fn parse_start_args(start_m: &ArgMatches) -> anyhow::Result<(Time, Tags)> {
        let time_arg = start_m.value_of("time");
        let tags_arg = start_m.values_of("tags");
        let mut tags_vec: Tags = vec![];
        let mut time = Time::Now;
        if let Some(time_str) = time_arg {
            if TimeTools::is_time(time_str) {
                time = TimeTools::time_from_str(time_str)?
            } else {
                tags_vec.push(String::from(time_str))
            }
        }
        if let Some(tags_str) = tags_arg {
            let tags = tags_str.map(String::from);
            for tag in tags {
                tags_vec.push(tag);
            }
        }
        Ok((time, tags_vec))
    }

    pub fn parse_stop_args(stop_m: &ArgMatches) -> anyhow::Result<(Time)> {
        let time_arg = stop_m.value_of("time");
        if let Some(time_str) = time_arg {
            TimeTools::time_from_str(time_str)
        } else {
            Ok(Time::Now)
        }
    }

    pub fn parse_summary_args(
        summary_m: &ArgMatches,
        clock: &dyn Clock,
    ) -> anyhow::Result<(AbsTime, AbsTime)> {
        if summary_m.is_present("yesterday") {
            return Ok(clock.yesterday_range());
        }
        if summary_m.is_present("lastweek") {
            return Ok(clock.last_week_range());
        }
        Ok(clock.today_range())
    }
}
