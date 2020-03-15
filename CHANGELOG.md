# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.0 (Unreleased)

## Added

* crate [chrono-english](https://docs.rs/chrono-english/) for time parsing see [commands](commands.md).
* more unit and integration tests
* `summary --week` option
* `summary range_start - range_end` syntax

## Fixed

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
