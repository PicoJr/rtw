[![rtw crate](https://img.shields.io/crates/v/rtw.svg)](https://crates.io/crates/rtw)
[![rtw documentation](https://docs.rs/rtw/badge.svg)](https://docs.rs/rtw)
[![GitHub license](https://img.shields.io/github/license/PicoJr/rtw)](https://github.com/PicoJr/rtw/blob/master/LICENSE)
[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2FPicoJr%2Frtw%2Fbadge&style=flat)](https://actions-badge.atrox.dev/PicoJr/rtw/goto)

# RTW - Rust Time Watcher

Command-line interface (CLI) time tracker.

This project is for educational purpose only. It is a _partial_ Rust implementation of [Timewarrior](https://github.com/GothenburgBitFactory/timewarrior).

> For a stable feature-rich CLI time tracker, please use Timewarrior: https://timewarrior.net/.

## Development

Development occurs on `dev`, releases are made on `master` branch.

Requires [Rust 1.40](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1400-2019-12-19)

## Install

```
cargo install rtw
```

Supported OS: Linux.

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

## Basic Usage

### Start tracking an activity

Example:
```bash
rtw start "learn rust"
```

Example output: 
```
Tracking learn rust
Started  2019-12-25T19:43:00
```

### Display current activity

``` bash
rtw
```

Example output: 
```
Tracking learn rust
Total    01:15:00
```

### Stop current activity

```bash
rtw stop
```

Example output: 
```
Recorded learn rust
Started 2019-12-25T19:43:00
Ended   2019-12-25T21:00:00
Total   01:17:000
```

### Display the day's activity summary

```bash
rtw summary
```

Example output: 
```
read the doc 2019-12-25T11:49:30 2019-12-25T11:53:36 00:04:246
eat cookies  2019-12-25T12:08:49 2019-12-25T12:12:14 00:03:204
```

### More?

For further details see [Full Usage](commands.md).

## TODO

- [x] start
- [x] stop
- [ ] track
- [x] continue
- [x] summary
- [ ] delete

## Implementation

RTW relies on json files for persistence.

Default location is the home (`~`) directory.

```
~/.rtw.json  # stores current activity
~/.rtwh.json # stores finished activity
```

**there is currently no file locking mechanism**: running several `rtw` commands at the same time
may lead to undefined behavior.
