[![rtw crate](https://img.shields.io/crates/v/rtw.svg)](https://crates.io/crates/rtw)
[![rtw documentation](https://docs.rs/rtw/badge.svg)](https://docs.rs/rtw)
[![GitHub license](https://img.shields.io/github/license/PicoJr/rtw)](https://github.com/PicoJr/rtw/blob/master/LICENSE)

|Branch|Status|
|------|------|
|[master](https://github.com/PicoJr/rtw/tree/master)|![Build Status](https://github.com/PicoJr/rtw/workflows/Rust/badge.svg?branch=master)|
|[dev](https://github.com/PicoJr/rtw/tree/dev)      |![Build Status](https://github.com/PicoJr/rtw/workflows/Rust/badge.svg?branch=dev)|

# RTW - Rust Time Watcher

Command-line interface (CLI) time tracker.

This project is for educational purpose only. It is a _partial_ Rust implementation of [Timewarrior](https://github.com/GothenburgBitFactory/timewarrior).

> For a stable feature-rich CLI time tracker, please use Timewarrior: https://timewarrior.net/.

## Development

Development occurs on `dev`, releases are made on `master` branch.

## Install

Supported OS: Linux.

### Cargo

```
cargo install rtw
```

### Build From Source

Clone and build from source:
```
git clone https://github.com/PicoJr/rtw.git
cd rtw
cargo build --release
```

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

## Implementation

RTW relies on json files for persistence.

Default location is the home (`~`) directory.

```
~/.rtw.json  # stores current activity
~/.rtwh.json # stores finished activity
```

**there is currently no file locking mechanism**: running several `rtw` commands at the same time
may lead to undefined behavior.
