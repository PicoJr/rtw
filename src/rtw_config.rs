extern crate config;

use self::config::FileFormat;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"
    {
        "timeline_colors": [[183,28,28], [26,35,126], [0,77,64], [38,50,56]]
    }
"#;

type RGB = (u8, u8, u8);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RTWConfig {
    pub storage_dir_path: PathBuf,
    pub timeline_colors: Vec<RGB>,
}

impl RTWConfig {
    pub fn default() -> Self {
        let home_dir = dirs::home_dir().expect("could not find home dir");
        RTWConfig {
            storage_dir_path: home_dir, // stores finished activities
            timeline_colors: vec![(183, 28, 28), (26, 35, 126), (0, 77, 64), (38, 50, 56)],
        }
    }
}

fn load_config_from_config_dir(
    config_dir: &PathBuf,
    default_config: RTWConfig,
) -> anyhow::Result<RTWConfig> {
    let mut settings = config::Config::default();
    let config_path = config_dir.join("rtw").join("rtw_config.json");
    let config_path_fallback = config_dir.join("rtw_config.json");
    settings
        .set_default(
            "storage_dir_path",
            default_config.storage_dir_path.to_str().unwrap(),
        )?
        .merge(config::File::from_str(DEFAULT_CONFIG, FileFormat::Json))?
        .merge(config::File::with_name(config_path.to_str().unwrap()).required(false))?
        .merge(config::File::with_name(config_path_fallback.to_str().unwrap()).required(false))?;
    let rtw_config: RTWConfig = settings.try_into()?;
    Ok(rtw_config)
}

pub fn load_config() -> anyhow::Result<RTWConfig> {
    match dirs::config_dir() {
        None => Ok(RTWConfig::default()),
        Some(config_dir) => load_config_from_config_dir(&config_dir, RTWConfig::default()),
    }
}

#[cfg(test)]
mod tests {
    use crate::rtw_config::{load_config_from_config_dir, RTWConfig};
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::str::FromStr;
    use tempfile::tempdir;

    #[test]
    // make sure the config file in `example` folder is valid
    fn example_config_valid() {
        let example_config = PathBuf::from_str("example/rtw_config.json").unwrap();
        let reader = File::open(example_config);
        let config: serde_json::Result<RTWConfig> = serde_json::from_reader(reader.unwrap());
        assert!(config.is_ok())
    }

    #[test]
    fn test_config_not_found_in_config_dir() {
        let test_config_dir = tempdir().expect("could not create temp directory");
        let test_dir_path = test_config_dir.path().to_path_buf();
        let config = load_config_from_config_dir(&test_dir_path, RTWConfig::default());
        assert_eq!(config.unwrap(), RTWConfig::default())
    }

    #[test]
    // .config/rtw_config.json
    fn test_config_found_in_config_dir() -> anyhow::Result<()> {
        let expected = PathBuf::from_str("/expected").unwrap();
        let test_config_dir = tempdir().expect("could not create temp directory");
        let mut tmp_config = File::create(test_config_dir.path().join("rtw_config.json"))?;
        writeln!(tmp_config, "{{\n\"storage_dir_path\": \"/expected\"\n}}")?;
        let config = load_config_from_config_dir(
            &test_config_dir.path().to_path_buf(),
            RTWConfig::default(),
        );
        assert_eq!(config.unwrap().storage_dir_path, expected);
        Ok(())
    }

    #[test]
    // .config/rtw/rtw_config.json
    fn test_config_found_in_sub_config_dir() -> anyhow::Result<()> {
        let expected = PathBuf::from_str("/expected").unwrap();
        let test_config_dir = tempdir().expect("could not create temp directory");
        let test_config_sub_dir = test_config_dir.path().join("rtw");
        fs::create_dir(test_config_sub_dir.clone()).expect("could not create temp/rtw directory");
        let mut tmp_config = File::create(test_config_sub_dir.join("rtw_config.json"))?;
        writeln!(tmp_config, "{{\n\"storage_dir_path\": \"/expected\"\n}}")?;
        let config = load_config_from_config_dir(
            &test_config_dir.path().to_path_buf(),
            RTWConfig::default(),
        );
        assert_eq!(config.unwrap().storage_dir_path, expected);
        Ok(())
    }
}
