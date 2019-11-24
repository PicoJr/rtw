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
}
