//! # RTW
//!
//! Command-line interface (CLI) time tracker.
//!
//! CLI usage is stable, underlying API is **not stable**.
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
