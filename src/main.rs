#[macro_use]
extern crate clap;

use crate::chrono_clock::ChronoClock;
use crate::json_storage::JsonStorage;
use crate::rtw_cli::{run, run_action};
use crate::service::Service;
use std::path::PathBuf;
use std::str::FromStr;

mod chrono_clock;
mod cli_helper;
mod json_storage;
mod rtw_cli;
mod rtw_config;
mod rtw_core;
mod service;
mod time_tools;
mod timeline;

fn main() -> anyhow::Result<()> {
    let cli_helper = cli_helper::ActivityCli {};
    let config = rtw_config::load_config()?;
    let clock = ChronoClock {};
    let app = cli_helper.get_app();
    let matches = app.get_matches();
    let storage_dir = match matches.value_of("directory") {
        None => config.storage_dir_path.clone(),
        Some(dir_str) => PathBuf::from_str(dir_str).expect("invalid directory"),
    };
    let current_activity_path = storage_dir.join(".rtw.json");
    let finished_activity_path = storage_dir.join(".rtwh.json");
    let mut service = Service::new(JsonStorage::new(
        current_activity_path,
        finished_activity_path,
    ));

    let action = run(matches, &mut service, &clock)?;
    run_action(action, &mut service, &clock, &config)
}
