extern crate rtw;

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use tempfile::tempdir;

    const NO_ACTIVE_TIME_TRACKING: &str = "There is no active time tracking.\n";
    const NO_FILTERED_DATA_FOUND: &str = "No filtered data found.\n";

    #[test]
    fn no_args() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .assert()
            .success()
            .stdout(NO_ACTIVE_TIME_TRACKING);
    }

    #[test]
    fn summary_none() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("summary")
            .assert()
            .success()
            .stdout(NO_FILTERED_DATA_FOUND);
    }

    #[test]
    fn summary_none_with_id() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("summary")
            .arg("--id")
            .assert()
            .success()
            .stdout(NO_FILTERED_DATA_FOUND);
    }

    #[test]
    fn summary_none_with_range() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("summary")
            .arg("09:00")
            .arg("-")
            .arg("10:00")
            .assert()
            .success()
            .stdout(NO_FILTERED_DATA_FOUND);
    }

    #[test]
    fn summary_something_with_range() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("track")
            .arg("09:00")
            .arg("-")
            .arg("10:00")
            .arg("foo")
            .assert()
            .success();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("summary")
            .arg("08:00")
            .arg("-")
            .arg("11:00")
            .assert()
            .success();
    }

    #[test]
    fn timeline_day_nothing() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("day")
            .assert()
            .success();
    }

    #[test]
    fn timeline_week_nothing() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("week")
            .assert()
            .success();
    }

    #[test]
    fn timeline_nothing() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("timeline")
            .arg("09:00")
            .arg("-")
            .arg("10:00")
            .assert()
            .success();
    }

    #[test]
    fn continue_none() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("continue")
            .assert()
            .success()
            .stdout("No activity to continue from.\n");
    }

    #[test]
    fn delete_none() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("delete")
            .arg("42")
            .assert()
            .success()
            .stdout("No activity found for id 42.\n");
    }

    #[test]
    fn delete_some() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("track")
            .arg("09:00")
            .arg("-")
            .arg("10:00")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Recorded foo"));
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("delete")
            .arg("0")
            .assert()
            .success()
            .stdout(predicates::str::contains("Deleted foo"));
    }

    #[test]
    fn start_now() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .arg("foo")
            .assert()
            .success();
    }

    #[test]
    fn start_then_stop() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Tracking foo"));
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("stop")
            .assert()
            .success()
            .stdout(predicates::str::contains("Recorded foo"));
    }

    #[test]
    fn start_then_stop_then_delete() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Tracking foo"));
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("stop")
            .assert()
            .success()
            .stdout(predicates::str::contains("Recorded foo"));
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("delete")
            .arg("0")
            .assert()
            .success()
            .stdout(predicates::str::contains("Deleted foo"));
    }

    #[test]
    fn track_date_missing_separator() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("track")
            .arg("2019-12-25T19:43:00")
            .arg("2019-12-25T19:45:00")
            .arg("foo")
            .assert()
            .failure();
    }

    #[test]
    fn track_date() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("track")
            .arg("2019-12-25T19:43:00")
            .arg("-")
            .arg("2019-12-25T19:45:00")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Recorded foo"));
    }

    #[test]
    fn track_relative() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("track")
            .arg("10 min ago")
            .arg("-")
            .arg("5 min ago")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Recorded foo"));
    }

    #[test]
    fn track_relative_time() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("track")
            .arg("09:00")
            .arg("-")
            .arg("10:00")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Recorded foo"));
    }

    #[test]
    fn track_relative_missing_end() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("track")
            .arg("10 min ago")
            .arg("-")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Recorded foo"));
    }

    #[test]
    fn track_relative_missing_start_and_end() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("track")
            .arg("-")
            .arg("foo")
            .assert()
            .failure();
    }

    #[test]
    fn start_nothing_now() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .assert()
            .failure();
    }

    #[test]
    fn start_nothing_15min_ago() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .arg("15min")
            .arg("ago")
            .assert()
            .failure();
    }

    #[test]
    fn start_foo_15min_ago() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .arg("15min")
            .arg("ago")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Tracking foo"));
    }

    #[test]
    fn start_foo_today_at_9() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .arg("09:00")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Tracking foo"));
    }

    #[test]
    fn start_foo_on_datetime() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .arg("2019-12-24T19:43:00")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Tracking foo"));
    }

    #[test]
    fn stop_nothing_now() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("stop")
            .assert()
            .success()
            .stdout(NO_ACTIVE_TIME_TRACKING);
    }

    #[test]
    fn stop_foo_5min_ago() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("start")
            .arg("10")
            .arg("min")
            .arg("ago")
            .arg("foo")
            .assert()
            .success()
            .stdout(predicates::str::contains("Tracking foo"));
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("stop")
            .arg("5")
            .arg("min")
            .arg("ago")
            .assert()
            .success()
            .stdout(predicates::str::contains("Recorded foo"));
    }
}
