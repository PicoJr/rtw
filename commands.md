# RTW Commands

## Start New Activity

start activity now: `rtw start write doc`

```
Tracking write doc
Started  2019-12-25T19:43:00
```

start activity 4 minutes ago: `rtw start 4m write doc`

```
Tracking write doc
Started  2019-12-25T19:39:00
```

start activity at a specific time: `rtw start 2019-12-24T19:43:00 write doc`

```
Tracking write doc
Started  2019-12-24T19:43:00
```

## Stop Current Activity

stop current activity now: `rtw stop`

```
Recorded write doc
Started 2019-12-25T19:43:00
Ended   2019-12-25T19:50:00
Total   00:07:00
```

stop current activity 4 minutes ago: `rtw stop 4m`

```
Recorded write doc
Started 2019-12-25T19:43:00
Ended   2019-12-25T19:46:00
Total   00:03:00
```

stop current activity at a specific time: `rtw stop 2019-12-25T19:45:00`

```
Recorded write doc
Started 2019-12-25T19:43:00
Ended   2019-12-25T19:45:00
Total   00:02:00
```

## Display Summary

display finished activities summary for today: `rtw summary`

```
write doc    2019-12-25T19:43:00 2019-12-25T19:45:00 00:03:000
```

display finished activities summary for yesterday: `rtw summary --yesterday`

```
write doc    2019-12-24T19:43:00 2019-12-24T19:45:00 00:03:000
```

display finished activities summary for last week: `rtw summary --lastweek`

```
write doc    2019-12-17T19:43:00 2019-12-17T19:45:00 00:03:000
```
