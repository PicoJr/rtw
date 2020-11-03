#[macro_use]
extern crate clap;

use crate::chrono_clock::ChronoClock;
use crate::cli_helper::get_app;
use crate::json_storage::JsonStorage;
use crate::rtw_cli::{dry_run_action, run, run_mutation};
use crate::rtw_config::{load_config, RTWConfig};
use crate::service::Service;
use std::path::PathBuf;
use std::str::FromStr;

mod chrono_clock;
mod cli_helper;
mod ical_export;
mod json_storage;
mod rtw_cli;
mod rtw_config;
mod rtw_core;
mod service;
mod time_tools;
mod timeline;

fn main() -> anyhow::Result<()> {
    let clock = ChronoClock {};
    let app = get_app();
    let matches = app.get_matches();
    let config = load_config()?;
    let config = if matches.is_present("default") {
        RTWConfig::default()
    } else {
        config
    };
    let config = if matches.is_present("overlap") {
        config.deny_overlapping(false)
    } else {
        config
    };
    let config = if matches.is_present("no_overlap") {
        config.deny_overlapping(true)
    } else {
        config
    };
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

    if cfg!(windows) {
        ansi_term::enable_ansi_support().unwrap_or(());
    }
    let action = run(&matches, &clock)?;
    let mutation = dry_run_action(action, &service, &clock, &config)?;
    if matches.is_present("dry-run") {
        println!("(dry-run) nothing done");
        Ok(())
    } else {
        run_mutation(mutation, &mut service, &config)
    }
}
