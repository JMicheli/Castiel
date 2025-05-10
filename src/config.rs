//! Configuration settings for the application.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardcastSettings {
  pub port: u16,
}

impl BoardcastSettings {
  pub fn initialize(config_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    // Write default settings file if one does not exist
    if !Path::new(config_path).exists() {
      tracing::info!("No Settings.toml found, writing default settings file");
      let toml_string = toml::to_string(&BoardcastSettings::default())?;
      std::fs::write(config_path, toml_string)?;
    }

    let file_content = std::fs::read_to_string(config_path)?;
    let settings: BoardcastSettings = toml::from_str(&file_content)?;

    tracing::info!("Initialized Boardcast settings: {settings:?}");
    Ok(settings)
  }
}

impl Default for BoardcastSettings {
  fn default() -> Self {
    Self { port: 3000 }
  }
}
