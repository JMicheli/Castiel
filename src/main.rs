//! Main entry point for Castiel.

mod config;
mod devices;
mod errors;
mod logging;
mod routes;

use std::path::Path;

use tokio::net::TcpListener;

use config::CastielSettings;

const DEFAULT_CONFIG_PATH: &str = "Settings.toml";

#[tokio::main]
async fn main() {
  // Load settings from file
  let settings =
    CastielSettings::initialize(Path::new(DEFAULT_CONFIG_PATH)).unwrap_or_else(|err| {
      tracing::warn!("Failed to load settings: {err}");
      CastielSettings::default()
    });

  logging::init_logging(&settings.log_level);
  tracing::info!("Launching Castiel server");

  // Create Axum Router
  let app = routes::create_router();

  // Bind TCP port indicated in settings
  let listener_addr = format!("127.0.0.1:{}", settings.port);
  let listener = TcpListener::bind(&listener_addr)
    .await
    .unwrap_or_else(|_| panic!("Failed to bind to {listener_addr}"));

  // Log and begin serving
  tracing::info!(
    "Listening on {}",
    listener
      .local_addr()
      .expect("Failed to retrieve local address")
  );
  axum::serve(listener, app).await.unwrap();
}
