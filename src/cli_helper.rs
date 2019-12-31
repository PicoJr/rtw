use clap::{App, Arg, ArgMatches, SubCommand};

use crate::time_tools::TimeTools;
use rtw::{ActivityId, Clock, DateTimeW, Tags, Time};
use std::str::FromStr;

pub struct ActivityCli {}

impl ActivityCli {
    pub fn get_app(&self) -> App {
        App::new("RTW")
            .version("0.2.0")
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
                SubCommand::with_name("track")
                    .about("Track a finished activity")
                    .arg(Arg::with_name("start").required(true).help("time"))
                    .arg(Arg::with_name("stop").required(true).help("time"))
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
                    )
                    .arg(
                        Arg::with_name("id")
                            .long("id")
                            .help("display activities id"),
                    ),
            )
            .subcommand(SubCommand::with_name("continue").about("Continue a finished activity"))
            .subcommand(
                SubCommand::with_name("delete")
                    .about("Delete activity")
                    .arg(Arg::with_name("id").required(true).help("activity id")),
            )
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

    pub fn parse_track_args(track_m: &ArgMatches) -> anyhow::Result<(Time, Time, Tags)> {
        let start_time_arg = track_m.value_of("start").expect("start time is required");
        let start_time = TimeTools::time_from_str(start_time_arg)?;
        let stop_time_arg = track_m.value_of("stop").expect("stop time is required");
        let stop_time = TimeTools::time_from_str(stop_time_arg)?;
        let tags = track_m
            .values_of("tags")
            .expect("at least one tag is required");
        let tags = tags.map(String::from);
        let mut tags_vec: Tags = vec![];
        for tag in tags {
            tags_vec.push(tag);
        }
        Ok((start_time, stop_time, tags_vec))
    }

    pub fn parse_stop_args(stop_m: &ArgMatches) -> anyhow::Result<Time> {
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
    ) -> anyhow::Result<((DateTimeW, DateTimeW), bool)> {
        let display_id = summary_m.is_present("id");
        if summary_m.is_present("yesterday") {
            return Ok((clock.yesterday_range(), display_id));
        }
        if summary_m.is_present("lastweek") {
            return Ok((clock.last_week_range(), display_id));
        }
        Ok((clock.today_range(), display_id))
    }

    pub fn parse_delete_args(delete_m: &ArgMatches) -> anyhow::Result<ActivityId> {
        let id_opt = delete_m
            .value_of("id")
            .map(|id_str| usize::from_str(id_str));
        if let Some(Ok(id)) = id_opt {
            Ok(id)
        } else {
            Err(anyhow::anyhow!("could not parse id"))
        }
    }
}
