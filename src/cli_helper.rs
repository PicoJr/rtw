use clap::{App, Arg, ArgMatches, SubCommand};

use crate::time_tools::TimeTools;
use rtw::{ActivityId, Clock, DateTimeW, Tag, Tags, Time};
use std::str::FromStr;

pub struct ActivityCli {}

fn split_time_clue_from_tags(tags: &[Tag], clock: &dyn Clock) -> (Time, Tags) {
    for at in (0..=tags.len()).rev() {
        let (possibly_time_clue, possibly_tags) = tags.split_at(at);
        let possibly_time_clue_joined: &str = &possibly_time_clue.join(" ");
        if TimeTools::is_time(possibly_time_clue_joined) {
            let time = TimeTools::time_from_str(possibly_time_clue_joined, clock).unwrap();
            return (time, possibly_tags.to_vec());
        }
    }
    (Time::Now, tags.to_vec())
}

impl ActivityCli {
    pub fn get_app(&self) -> App {
        App::new("RTW")
            .version("1.0.0")
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
                    .arg(
                        Arg::with_name("tags")
                            .multiple(true)
                            .required(true)
                            .help(concat!(
                                "optional time clue followed by at least 1 tag\n",
                                "e.g '4 min ago foo' or '09:00 foo' or 'foo' "
                            )),
                    ),
            )
            .subcommand(
                SubCommand::with_name("track")
                    .about("Track a finished activity")
                    .arg(Arg::with_name("start").required(true).help("time"))
                    .arg(Arg::with_name("stop").required(true).help("time"))
                    .arg(Arg::with_name("tags").multiple(true).help("tags")),
            )
            .subcommand(
                SubCommand::with_name("stop").about("Stop activity").arg(
                    Arg::with_name("time")
                        .multiple(true)
                        .required(false)
                        .help(concat!(
                            "optional time clue e.g. 4min ago\n",
                            "current time is used when omitted"
                        )),
                ),
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

    pub fn parse_start_args(
        start_m: &ArgMatches,
        clock: &dyn Clock,
    ) -> anyhow::Result<(Time, Tags)> {
        let values_arg = start_m.values_of("tags"); // optional time clue, tags
        if let Some(values) = values_arg {
            let values: Tags = values.map(String::from).collect();
            let (time, tags) = split_time_clue_from_tags(&values, clock);
            return if tags.is_empty() {
                Err(anyhow::anyhow!("no tags provided"))
            } else {
                Ok((time, tags))
            };
        }
        Err(anyhow::anyhow!("neither time clue nor tags provided")) // it should be prevented by clap
    }

    pub fn parse_track_args(
        track_m: &ArgMatches,
        clock: &dyn Clock,
    ) -> anyhow::Result<(Time, Time, Tags)> {
        let start_time_arg = track_m.value_of("start").expect("start time is required");
        let start_time = TimeTools::time_from_str(start_time_arg, clock)?;
        let stop_time_arg = track_m.value_of("stop").expect("stop time is required");
        let stop_time = TimeTools::time_from_str(stop_time_arg, clock)?;
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

    pub fn parse_stop_args(stop_m: &ArgMatches, clock: &dyn Clock) -> anyhow::Result<Time> {
        let time_arg = stop_m.values_of("time");
        if let Some(values) = time_arg {
            let values: Vec<String> = values.map(String::from).collect();
            let time_str = values.join(" ");
            TimeTools::time_from_str(&time_str, clock)
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

#[cfg(test)]
mod tests {
    use crate::chrono_clock::ChronoClock;
    use crate::cli_helper::split_time_clue_from_tags;
    use rtw::{Tags, Time};

    #[test]
    // rtw start
    fn test_split_time_clue_from_tags_0_0() {
        let clock = ChronoClock {};
        let values: Tags = vec![];
        let (time, tags) = split_time_clue_from_tags(&values, &clock);
        assert_eq!(Time::Now, time);
        assert!(tags.is_empty());
    }

    #[test]
    // rtw start foo
    fn test_split_time_clue_from_tags_0_1() {
        let clock = ChronoClock {};
        let values: Tags = vec![String::from("foo")];
        let (time, tags) = split_time_clue_from_tags(&values, &clock);
        assert_eq!(Time::Now, time);
        assert_eq!(tags, values);
    }

    #[test]
    // rtw start foo bar
    fn test_split_time_clue_from_tags_0_2() {
        let clock = ChronoClock {};
        let values: Tags = vec![String::from("foo"), String::from("bar")];
        let (time, tags) = split_time_clue_from_tags(&values, &clock);
        assert_eq!(Time::Now, time);
        assert_eq!(tags, values);
    }

    #[test]
    // rtw start 1 h ago
    fn test_split_time_clue_from_tags_3_0() {
        let clock = ChronoClock {};
        let values: Tags = vec![String::from("1"), String::from("h"), String::from("ago")];
        let (time, tags) = split_time_clue_from_tags(&values, &clock);
        assert_ne!(Time::Now, time);
        assert!(tags.is_empty());
    }

    #[test]
    // rtw start 1 h ago foo
    fn test_split_time_clue_from_tags_3_1() {
        let clock = ChronoClock {};
        let values: Tags = vec![
            String::from("1"),
            String::from("h"),
            String::from("ago"),
            String::from("foo"),
        ];
        let (time, tags) = split_time_clue_from_tags(&values, &clock);
        assert_ne!(Time::Now, time);
        assert_eq!(tags, vec![String::from("foo")]);
    }
}
