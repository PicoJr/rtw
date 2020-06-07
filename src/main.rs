use rtw::chrono_clock::ChronoClock;
use rtw::json_storage::JsonStorage;
use rtw::rtw_cli::{run, run_action};
use rtw::service::Service;
use rtw::{cli_helper, rtw_config};
use std::path::PathBuf;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let cli_helper = cli_helper::ActivityCli {};
    let config = rtw_config::load_config()?;
    let clock = ChronoClock {};
    let app = cli_helper.get_app();
    let matches = app.get_matches();
    let storage_dir = match matches.value_of("directory") {
        None => config.storage_dir_path.clone(),
        Some(dir_str) => PathBuf::from_str(dir_str).expect("invalid directory"),
    };
    let current_activity_path = storage_dir.join(".rtw.json");
    let finished_activity_path = storage_dir.join(".rtwh.json");
    let mut service = Service::new(JsonStorage::new(
        current_activity_path,
        finished_activity_path,
    ));

    let action = run(matches, &mut service, &clock)?;
    // skipping this should be the same as a dry-run.
    run_action(action, &mut service, &clock, &config)
}
