# RTW

CLI Time Tracker.

A simplified [timewarrior](https://github.com/GothenburgBitFactory/timewarrior) in Rust.

This project is for educational purpose only, for a stable feature-rich CLI time tracker please use timewarrior: https://timewarrior.net/.

## Usage

start a new activity: `rtw start "learn rust"`

```bash
Tracking learn rust
Started  2019-12-25T19:43:00
```

display current activity: `rtw`

```
Tracking learn rust
Total    01:15:00
```

stop current activity: `rtw stop`

```
Recorded learn rust
Started 2019-12-25T19:43:00
Ended   2019-12-22T21:00:00
Total   01:17:000
```

## Commands

- [x] start
- [x] stop
- [ ] track
- [ ] continue
- [ ] summary
- [ ] delete