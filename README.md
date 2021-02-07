[![rtw crate](https://img.shields.io/crates/v/rtw.svg)](https://crates.io/crates/rtw)
[![rtw documentation](https://docs.rs/rtw/badge.svg)](https://docs.rs/rtw)
[![GitHub license](https://img.shields.io/github/license/PicoJr/rtw)](https://github.com/PicoJr/rtw/blob/master/LICENSE)

|Branch|Status|
|------|------|
|[master](https://github.com/PicoJr/rtw/tree/master)|![Build Status](https://github.com/PicoJr/rtw/workflows/Rust/badge.svg?branch=master)|
|[dev](https://github.com/PicoJr/rtw/tree/dev)      |![Build Status](https://github.com/PicoJr/rtw/workflows/Rust/badge.svg?branch=dev)|

# RTW - Rust Time Watcher

Command-line interface (CLI) time tracker.

CLI usage is stable, underlying API is **not stable**.

> Note: This software is built specifically as a productivity tool for myself,
> not as a consumer resource. I cannot commit a large amount of time to maintaining this
> software but I'll do my best to provide support if something fails =).

This project is heavily inspired from [Timewarrior](https://github.com/GothenburgBitFactory/timewarrior).

> For a stable feature-rich CLI time tracker, please use Timewarrior: https://timewarrior.net/.

## Why another time tracker tool?

1. learn Rust
2. I once lost a month worth of data with another time tracker tool (database corruption)...never again!

## Install

Supported OS: Linux, MacOS, Windows

CI runs on `ubuntu-latest`, `macos-latest`, `windows-latest`.

Note: Windows support is only experimental. Some features may not be supported on Windows.

### Cargo

```
cargo install rtw
```

### Build From Source

rtw compiles with Rust 1.42.0 (stable) or newer.

Clone and build from source:
```
git clone https://github.com/PicoJr/rtw.git
cd rtw
cargo build --release
```

### From binaries (Linux only)

Download the corresponding archive from the [Release page](https://github.com/picojr/rtw/releases).

### Shell Completion (Bash, Zsh, Fish, Powershell, Elvish)

Please see [shell completion](shell-completion.md).

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

### Display a timeline for the day

```bash
rtw day
```

Example output (YMMV):

![timeline](img/day.png)

### More?

For further details see [Full Usage](commands.md).

## Configuration

RTW doesn't create the config file for you, but it looks for one in the following locations (in this order):

1. `$XDG_CONFIG_HOME/rtw/rtw_config.json`
2. `$HOME/.config/rtw/rtw_config.json`
3. `$XDG_CONFIG_HOME/.config/rtw_config.json`
4. `$HOME/.config/rtw_config.json`

see `example` folder for a default config file.

## Development

Development occurs on `dev`, releases are made on `master` branch.

## Implementation

RTW relies on json files for persistence.

Default location is the home (`~`) directory.

```
~/.rtw.json  # stores current activity
~/.rtwh.json # stores finished activities
```

**there is currently no file locking mechanism**: running several `rtw` commands at the same time
may lead to undefined behavior.

## Similar Tools

* [timewarrior](https://github.com/GothenburgBitFactory/timewarrior)
* [watson](https://github.com/TailorDev/Watson)
* [jobrog](https://github.com/dfhoughton/jobrog)
* [doug](https://github.com/chdsbd/doug)
* ...
