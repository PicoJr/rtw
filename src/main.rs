extern crate clap;

use crate::chrono_clock::ChronoClock;
use crate::json_current::JsonCurrentActivityRepository;
use crate::json_finished::JsonFinishedActivityRepository;
use crate::rtw_cli::RTW;
use crate::service::Service;
use std::path::PathBuf;
use std::str::FromStr;

mod chrono_clock;
mod cli_helper;
mod json_current;
mod json_finished;
mod rtw_cli;
mod service;
mod time_tools;

fn main() -> anyhow::Result<()> {
    let c = cli_helper::ActivityCli {};
    let clock = ChronoClock {};
    let app = c.get_app();
    let matches = app.get_matches();
    let storage_dir = match matches.value_of("directory") {
        None => dirs::home_dir().expect("could not find home dir"),
        Some(dir_str) => PathBuf::from_str(dir_str).expect("invalid directory"),
    };
    let current_activity_path = storage_dir.join(".rtw.json");
    let finished_activity_path = storage_dir.join(".rtwh.json");
    let service = Service::new(
        JsonFinishedActivityRepository::new(finished_activity_path),
        JsonCurrentActivityRepository::new(current_activity_path),
    );

    let mut rtw_cli = RTW::new(clock, service);
    rtw_cli.run(matches)
}
