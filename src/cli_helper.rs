//! CLI parsing helpers and clap App.
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::rtw_core::clock::{Clock, Time};
use crate::rtw_core::datetimew::DateTimeW;
use crate::rtw_core::{ActivityId, Tags};
use crate::time_tools::TimeTools;
use std::str::FromStr;

// 09:00 foo -> (09:00, foo)
// foo -> (Now, foo)
// last friday 8pm foo -> (last friday 8pm, foo)
fn split_time_clue_from_tags(tokens: &[String], clock: &dyn Clock) -> (Time, Tags) {
    for at in (0..=tokens.len()).rev() {
        let (possibly_time_clue, possibly_tags) = tokens.split_at(at);
        let possibly_time_clue_joined: &str = &possibly_time_clue.join(" ");
        if TimeTools::is_time(possibly_time_clue_joined) {
            let time = TimeTools::time_from_str(possibly_time_clue_joined, clock).unwrap();
            return (time, possibly_tags.to_vec());
        }
    }
    (Time::Now, tokens.to_vec())
}

// "09:00 - 10:00 foo" -> (09:00, 10:00, foo)
fn split_time_range_from_tags(
    tokens: &[String],
    clock: &dyn Clock,
) -> anyhow::Result<(Time, Time, Tags)> {
    let separator = "-";
    let sp = tokens.splitn(2, |e| e == separator);
    let sp: Vec<&[String]> = sp.collect();
    match sp.as_slice() {
        [range_start, range_end_and_tags] => {
            let range_start_maybe = TimeTools::time_from_str(&range_start.join(" "), clock);
            let (range_end, activity_tags) = split_time_clue_from_tags(&range_end_and_tags, clock);
            match range_start_maybe {
                Ok(range_start) => Ok((range_start, range_end, activity_tags)),
                Err(e) => Err(anyhow::anyhow!(e)),
            }
        }
        _ => Err(anyhow::anyhow!(
            "missing ' - ' between range start and range end? "
        )),
    }
}

// 09:00 - 10:00 -> (09:00, 10:00)
// 09:00 - -> (09:00, Now)
fn split_time_range(tokens: &[String], clock: &dyn Clock) -> anyhow::Result<(Time, Time)> {
    let separator = "-";
    let sp = tokens.splitn(2, |e| e == separator);
    let sp: Vec<&[String]> = sp.collect();
    match sp.as_slice() {
        [range_start, range_end] => {
            let range_start_maybe = TimeTools::time_from_str(&range_start.join(" "), clock);
            let range_end_maybe = if range_end.is_empty() {
                Ok(Time::Now)
            } else {
                TimeTools::time_from_str(&range_end.join(" "), clock)
            };
            match (range_start_maybe, range_end_maybe) {
                (Ok(range_start), Ok(range_end)) => Ok((range_start, range_end)),
                _ => Err(anyhow::anyhow!("invalid range")),
            }
        }
        _ => Err(anyhow::anyhow!(
            "missing ' - ' between range start and range end? "
        )),
    }
}

pub fn get_app() -> App<'static, 'static> {
    App::new("RTW")
        .version(crate_version!())
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
        .arg(
            Arg::with_name("dry-run")
                .short("n")
                .long("dry")
                .required(false)
                .help("dry run don't write anything to the filesystem"),
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("Start new activity")
                .arg(
                    Arg::with_name("tokens")
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
                .arg(
                    Arg::with_name("tokens")
                        .multiple(true)
                        .required(true)
                        .help(concat!(
                            "interval time clue followed by at least 1 tag\n",
                            "start - end tags...\n",
                            "e.g '09:00 - 10:00 foo' "
                        )),
                ),
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
                    Arg::with_name("tokens")
                        .multiple(true)
                        .required(false)
                        .conflicts_with_all(&["yesterday", "lastweek", "week"])
                        .help(concat!(
                            "optional interval time clue\n",
                            "start - end\n",
                            "e.g '09:00 - 10:00' "
                        )),
                )
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
                    Arg::with_name("week")
                        .long("week")
                        .help("activities done this week"),
                )
                .arg(
                    Arg::with_name("id")
                        .long("id")
                        .help("display activities id"),
                ),
        )
        .subcommand(SubCommand::with_name("continue").about("Continue a finished activity"))
        .subcommand(SubCommand::with_name("day").about("Display the current day as a timeline"))
        .subcommand(SubCommand::with_name("week").about("Display the current week as a timeline"))
        .subcommand(
            SubCommand::with_name("timeline")
                .about("Display finished activities timeline")
                .arg(
                    Arg::with_name("tokens")
                        .multiple(true)
                        .required(false)
                        .help(concat!(
                            "optional interval time clue\n",
                            "start - end\n",
                            "e.g 'last monday - now' "
                        )),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete activity")
                .arg(Arg::with_name("id").required(true).help("activity id")),
        )
        .subcommand(SubCommand::with_name("cancel").about("cancel current activity"))
}

pub fn parse_start_args(start_m: &ArgMatches, clock: &dyn Clock) -> anyhow::Result<(Time, Tags)> {
    let values_arg = start_m.values_of("tokens"); // optional time clue, tags
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
    let values_arg = track_m
        .values_of("tokens")
        .expect("start time, end time and at least 1 tag required");
    let values: Tags = values_arg.map(String::from).collect();
    split_time_range_from_tags(&values, clock)
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
    let values_arg = summary_m.values_of("tokens");
    if let Some(values) = values_arg {
        let values: Vec<String> = values.map(String::from).collect();
        let range_maybe = split_time_range(&values, clock);
        return match range_maybe {
            Ok((range_start, range_end)) => {
                let range_start = clock.date_time(range_start);
                let range_end = clock.date_time(range_end);
                Ok(((range_start, range_end), display_id))
            }
            Err(e) => Err(anyhow::anyhow!(e)),
        };
    }
    let range = {
        if summary_m.is_present("yesterday") {
            clock.yesterday_range()
        } else if summary_m.is_present("lastweek") {
            clock.last_week_range()
        } else if summary_m.is_present("week") {
            clock.this_week_range()
        } else {
            clock.today_range()
        }
    };
    Ok((range, display_id))
}

pub fn parse_timeline_args(
    timeline_m: &ArgMatches,
    clock: &dyn Clock,
) -> anyhow::Result<((DateTimeW, DateTimeW), bool)> {
    let display_id = timeline_m.is_present("id");
    let values_arg = timeline_m.values_of("tokens");
    if let Some(values) = values_arg {
        let values: Vec<String> = values.map(String::from).collect();
        let range_maybe = split_time_range(&values, clock);
        match range_maybe {
            Ok((range_start, range_end)) => {
                let range_start = clock.date_time(range_start);
                let range_end = clock.date_time(range_end);
                Ok(((range_start, range_end), display_id))
            }
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    } else {
        Ok((clock.today_range(), display_id))
    }
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

#[cfg(test)]
mod tests {
    use crate::chrono_clock::ChronoClock;
    use crate::cli_helper::{
        split_time_clue_from_tags, split_time_range, split_time_range_from_tags,
    };
    use crate::rtw_core::clock::Time;
    use crate::rtw_core::Tags;
    use crate::time_tools::TimeTools;

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
        let tokens: Vec<String> = vec![
            String::from("1"),
            String::from("h"),
            String::from("ago"),
            String::from("foo"),
        ];
        let (time, tags) = split_time_clue_from_tags(&tokens, &clock);
        assert_ne!(Time::Now, time);
        assert_eq!(tags, vec![String::from("foo")]);
    }

    #[test]
    // rtw track 09:00 - 10:00 foo
    fn test_split_time_range_from_tags_1_1_1() {
        let clock = ChronoClock {};
        let tokens: Vec<String> = vec![
            String::from("09:00"),
            String::from("-"),
            String::from("10:00"),
            String::from("foo"),
        ];
        let time_range_and_tags = split_time_range_from_tags(&tokens, &clock);
        assert!(time_range_and_tags.is_ok());
    }

    #[test]
    // rtw summary 09:00 - 10:00
    fn test_split_range_1_1() {
        let clock = ChronoClock {};
        let tokens: Vec<String> = vec![
            String::from("09:00"),
            String::from("-"),
            String::from("10:00"),
        ];
        let time_range = split_time_range(&tokens, &clock);
        assert!(time_range.is_ok());
        let time_range = time_range.unwrap();
        assert_eq!(
            time_range.0,
            TimeTools::time_from_str("09:00", &clock).unwrap()
        );
        assert_eq!(
            time_range.1,
            TimeTools::time_from_str("10:00", &clock).unwrap()
        );
    }

    #[test]
    // rtw summary 09:00 -
    fn test_split_range_1_0() {
        let clock = ChronoClock {};
        let tokens: Vec<String> = vec![String::from("09:00"), String::from("-")];
        let time_range = split_time_range(&tokens, &clock);
        assert!(time_range.is_ok());
        assert_eq!(time_range.unwrap().1, Time::Now)
    }
}
