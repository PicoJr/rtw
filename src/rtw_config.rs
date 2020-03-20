extern crate config;

use std::path::PathBuf;
use std::str::FromStr;

pub struct RTWConfig {
    pub storage_dir_path: PathBuf,
}

impl RTWConfig {
    pub fn default() -> Self {
        let home_dir = dirs::home_dir().expect("could not find home dir");
        RTWConfig {
            storage_dir_path: home_dir, // stores finished activities
        }
    }
}

pub fn load_config() -> anyhow::Result<RTWConfig> {
    let default_config = RTWConfig::default();
    let mut settings = config::Config::default();
    match dirs::config_dir() {
        Some(config_dir) => {
            let config_path = config_dir.join("rtw").join("rtw_config.json");
            let config_path_fallback = config_dir.join("rtw_config.json");
            let settings = settings
                .set_default(
                    "storage_dir_path",
                    default_config.storage_dir_path.to_str().unwrap(),
                )?
                .merge(config::File::with_name(config_path.to_str().unwrap()).required(false))?
                .merge(
                    config::File::with_name(config_path_fallback.to_str().unwrap()).required(false),
                )?;
            let storage_dir_path = settings.get_str("storage_dir_path")?;
            Ok(RTWConfig {
                storage_dir_path: PathBuf::from_str(storage_dir_path.as_str())?,
            })
        }
        None => Ok(default_config),
    }
}
