# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

* `CHANGELOG.md`
* `commands.md`
* `summary --lastweek` option
* github action
* badges

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