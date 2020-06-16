# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.1](https://crates.io/crates/rtw/1.3.1) Jun 16, 2020

* Fix timeline crash when activity is too short to be displayed (#28)

## [1.3.0](https://crates.io/crates/rtw/1.3.0) Jun 13, 2020

* Add multiline timeline

## [1.2.2](https://crates.io/crates/rtw/1.2.2) Jun 09, 2020

* Add `-n` dry-run option.

## [1.2.1](https://crates.io/crates/rtw/1.2.1) Jun 07, 2020

* Add warning: CLI usage stable but not `lib.rs` content.
* Fix doc.rs build issue (restore `lib.rs`).

## [1.2.0](https://crates.io/crates/rtw/1.2.0) Jun 07, 2020

* add `cancel` subcommand.
* deny overlapping activities
* add `timeline` subcommand.
* timeline colors can be configured in `rtw_config.json`
* add `day` subcommand (display timeline for the current day)
* add `week` subcommand (display timeline for the current week)

## [1.1.0](https://crates.io/crates/rtw/1.1.0) Mar 22, 2020

### Added

* add config using [config-rs](https://docs.rs/crate/config/0.10.1).

### Changed

* activities title are no longer truncated in summary

### Github CI

* Add platforms: `macos-latest`, `windows-latest` (see [rust.yml](.github/workflows/rust.yml)).

## [1.0.0](https://crates.io/crates/rtw/1.0.0) Mar 16, 2020

### Added

* crate [chrono-english](https://docs.rs/chrono-english/) for time parsing see [commands](commands.md).
* more unit and integration tests
* `summary --week` option
* `summary range_start - range_end` syntax

### Fixed

* Duration display bug: 1h was displayed as `01:60:3600` instead of `01:00:00`

### Breaking API Changes

`rtw` now uses the crate [chrono-english](https://docs.rs/chrono-english/) for time parsing.

As a result `rtw` now support the following [formats](https://docs.rs/chrono-english/#supported-formats) when supplying time hints.

The following syntax are not supported anymore:

* `rtw start 4m foo`, use `rtw start 4m ago foo` instead.
* `rtw stop 4m`, use `rtw stop 4m ago` instead.
* `rtw track 2019-12-25T19:43:00 2019-12-25T19:45:00 write doc`, use `rtw track 2019-12-25T19:43:00 - 2019-12-25T19:45:00 write doc` instead

## [0.2.1](https://crates.io/crates/rtw/0.2.1) Mar 8, 2020

### Fixed

* fix cargo-audit warning on `quote:1.0.2` being yanked

### Removed

* ram-only implementations

## [0.2.0](https://crates.io/crates/rtw/0.2.0) Dec 31, 2019

### Added

* `track` command
* `delete` command
* `summary --id` option
* doc test
* `continue` command
* `CHANGELOG.md`
* `commands.md`
* `summary --lastweek` option
* github action
* badges

### Changed

* `AbsTime` renamed to `DateTimeW`
* `ActiveActivity` renamed to `OngoingActivity`

### Fixed

* `summary` output is now sorted by start date
* `tempfile` and `assert_cmd` no longer required for build
* CLI version now matches `Cargo.toml` version

## [0.1.1](https://crates.io/crates/rtw/0.1.1) Dec 26, 2019

### Added

* repository url in `Cargo.toml`

## [0.1.0](https://crates.io/crates/rtw/0.1.0) Dec 26, 2019

### Added

* `start` command
* `stop` command
* `summary` command
