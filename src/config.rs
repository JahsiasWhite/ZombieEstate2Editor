use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub game_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, std::io::Error> {
        let config_file = Path::new("config.json");
        if config_file.exists() {
            let content = std::fs::read_to_string(config_file)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Default::default())
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_file = Path::new("config.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(config_file, content)
    }
}