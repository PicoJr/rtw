extern crate rtw;

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use tempfile::tempdir;

    #[test]
    fn no_args() {
        let test_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_dir.path().to_str().unwrap();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .assert()
            .success()
            .stdout("There is no active time tracking.\n");
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
            .stdout("No filtered data found.\n");
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
            .success();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("stop")
            .assert()
            .success();
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
            .success();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("stop")
            .assert()
            .success();
        let mut cmd = Command::cargo_bin("rtw").unwrap();
        cmd.arg("-d")
            .arg(test_dir_path)
            .arg("delete")
            .arg("0")
            .assert()
            .success();
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
            .arg("2019-12-25T19:45:00")
            .arg("foo")
            .assert()
            .success();
    }
}
