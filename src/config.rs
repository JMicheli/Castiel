//! Configuration settings for the application.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardcastSettings {
  pub port: u16,
}

impl BoardcastSettings {
  pub fn initialize(config_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    if !Path::new(config_path).exists() {
      let default_settings = Self { port: 3000 };
      let toml_string = toml::to_string(&default_settings)?;
      std::fs::write(config_path, toml_string)?;
    }

    let file_content = std::fs::read_to_string(config_path)?;
    let settings: BoardcastSettings = toml::from_str(&file_content)?;

    Ok(settings)
  }
}

impl Default for BoardcastSettings {
  fn default() -> Self {
    Self { port: 3000 }
  }
}
