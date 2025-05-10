//! Configuration settings for the application.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct CastielSettings {
  pub port: u16,
  pub log_level: String,
}

impl Default for CastielSettings {
  fn default() -> Self {
    Self {
      port: 3000,
      log_level: "INFO".to_string(),
    }
  }
}

impl CastielSettings {
  /// Used to initialize settings from the file at `config_path`. It will create a default config
  /// at that path if none exists.
  ///
  /// Because this function is intended to be called before logging is initialized, it uses
  /// println! macros instead of the tracing crate used elsewhere.
  pub fn initialize(config_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    // Write default settings file if one does not exist
    if !Path::new(config_path).exists() {
      println!("No Settings.toml found, writing default settings file");
      let toml_string = toml::to_string(&Self::default())?;
      std::fs::write(config_path, toml_string)?;
    }

    let file_content = std::fs::read_to_string(config_path)?;
    let settings: Self = toml::from_str(&file_content)?;

    // Possibly log initialization depending on level
    if settings.log_level.eq_ignore_ascii_case("INFO")
      || settings.log_level.eq_ignore_ascii_case("TRACE")
    {
      println!("Initialized Castiel settings: {settings:?}");
    }

    Ok(settings)
  }
}
