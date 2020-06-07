//! # RTW
//!
//! Command-line interface (CLI) time tracker.
//!
//! This project is heavily inspired from [Timewarrior](https://github.com/GothenburgBitFactory/timewarrior).
//!
//! For a stable feature-rich CLI time tracker, please use Timewarrior: <https://timewarrior.net/>.
//!
//! ## Design
//!
//! * Activities are stored inside a `Storage`.
//! * An `ActivityService` provides the logic above a storage.
//! * `rtw_cli::run` translates CLI args to actions (`RTWAction`).
//! * `rtw_cli::run_action` performs actions `RTWAction` by calling the service.
//!
//! ## Tests
//!
//! RTW has both unit and integration tests.

#[macro_use]
extern crate clap;

pub mod chrono_clock;
pub mod cli_helper;
pub mod json_storage;
pub mod rtw_cli;
pub mod rtw_config;
pub mod rtw_core;
pub mod service;
pub mod time_tools;
mod timeline;
