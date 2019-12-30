# RTW Commands

<!--ts-->
   * [RTW Commands](#rtw-commands)
      * [Start New Activity](#start-new-activity)
         * [Start tracking an activity now](#start-tracking-an-activity-now)
         * [Start tracking an activity 4 minutes ago](#start-tracking-an-activity-4-minutes-ago)
         * [Start tracking an activity at a specific time](#start-tracking-an-activity-at-a-specific-time)
      * [Stop Current Activity](#stop-current-activity)
         * [Stop current activity now](#stop-current-activity-now)
         * [Stop current activity 4 minutes ago](#stop-current-activity-4-minutes-ago)
         * [Stop current activity at a specific time](#stop-current-activity-at-a-specific-time)
      * [Display Summary](#display-summary)
         * [Display finished activities summary for today](#display-finished-activities-summary-for-today)
         * [Display finished activities summary for yesterday](#display-finished-activities-summary-for-yesterday)
         * [Display finished activities summary for last week](#display-finished-activities-summary-for-last-week)
         * [Display finished activities id](#display-finished-activities-id)
      * [Continue Activity](#continue-activity)
         * [Continue last finished activity](#continue-last-finished-activity)
      * [Delete Activity](#delete-activity)
         * [Delete Activity with id](#delete-activity-with-id)
      * [Track a finished activity](#track-a-finished-activity)
         * [Track a finished activity with dates](#track-a-finished-activity-with-dates)

<!--te-->
Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)

## Start New Activity

### Start tracking an activity now
 
Example:
 ```
 rtw start write doc
```

Example output:
```
Tracking write doc
Started  2019-12-25T19:43:00
```

### Start tracking an activity 4 minutes ago

Example:
```
rtw start 4m write doc
```

Example output:
```
Tracking write doc
Started  2019-12-25T19:39:00
```

### Start tracking an activity at a specific time

Example:
```
rtw start 2019-12-24T19:43:00 write doc
```

Example output:
```
Tracking write doc
Started  2019-12-24T19:43:00
```

## Stop Current Activity

### Stop current activity now

Example:
```
rtw stop
```

Example output:
```
Recorded write doc
Started 2019-12-25T19:43:00
Ended   2019-12-25T19:50:00
Total   00:07:00
```

### Stop current activity 4 minutes ago

Example:
```
rtw stop 4m
```

Example output:
```
Recorded write doc
Started 2019-12-25T19:43:00
Ended   2019-12-25T19:46:00
Total   00:03:00
```

### Stop current activity at a specific time

Example:
```
rtw stop 2019-12-25T19:45:00
```

Example output:
```
Recorded write doc
Started 2019-12-25T19:43:00
Ended   2019-12-25T19:45:00
Total   00:02:00
```

## Display Summary

### Display finished activities summary for today

Example:
```
rtw summary
```

Example output:
```
write doc    2019-12-25T19:43:00 2019-12-25T19:45:00 00:03:000
```

### Display finished activities summary for yesterday

Example:
```
rtw summary --yesterday
```

Example output:
```
write doc    2019-12-24T19:43:00 2019-12-24T19:45:00 00:03:000
```

### Display finished activities summary for last week

Example:
```
rtw summary --lastweek
```

Example output:
```
write doc    2019-12-17T19:43:00 2019-12-17T19:45:00 00:03:000
```

### Display finished activities id

Example:
```
rtw summary --id
```

Example output:
```
 2 foo          2019-12-25T17:43:00 2019-12-25T17:44:00 00:01:00
 1 another foo  2019-12-25T18:43:00 2019-12-25T18:44:00 00:01:00
 0 bar          2019-12-25T19:43:00 2019-12-25T19:44:00 00:01:00
```

> id 0 = last finished activity

## Continue Activity

### Continue last finished activity

Example:
```
rtw continue
```

Example output:
```
Tracking write doc
Total    00:00:00
```

## Delete Activity

### Delete Activity with id

Example:
```
rtw delete 1
```

Example output:
```
Deleted write doc
Started 2019-12-25T19:43:00
Ended   2019-12-25T19:45:00
Total   00:02:00
```

## Track a finished activity

### Track a finished activity with dates

Example:
```
rtw track  2019-12-25T19:43:00 2019-12-25T19:45:00 write doc
```

Example ouput
```
Recorded write doc
Started 2019-12-25T19:43:00
Ended   2019-12-25T19:45:00
Total   00:02:00
```