use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Default)]
pub struct Config {
    pub tmpdir: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let mut config = Self::from_file().unwrap_or_default();

        if let Ok(tmpdir) = env::var("FIG_TMPDIR") {
            config.tmpdir = Some(tmpdir);
        }

        config
    }

    fn from_file() -> Option<Self> {
        let config_dir = env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))?;

        let config_path = config_dir.join("fig").join("config.toml");

        let contents = fs::read_to_string(config_path).ok()?;
        toml::from_str(&contents).ok()
    }
}

