//! Main entry point for Boardcast.

mod config;
mod devices;
mod routes;

use std::path::Path;

use tokio::net::TcpListener;

use config::BoardcastSettings;

const DEFAULT_CONFIG_PATH: &str = "Settings.toml";

#[tokio::main]
async fn main() {
  // Load settings from file
  let settings =
    BoardcastSettings::initialize(Path::new(DEFAULT_CONFIG_PATH)).unwrap_or_else(|err| {
      tracing::warn!("Failed to load settings: {err}");
      BoardcastSettings::default()
    });

  let app = routes::create_router();

  let listener_addr = format!("127.0.0.1:{}", settings.port);
  let listener = TcpListener::bind(&listener_addr)
    .await
    .unwrap_or_else(|_| panic!("Failed to bind to {listener_addr}"));

  tracing::info!(
    "Listening on {}",
    listener
      .local_addr()
      .expect("Failed to retrieve local address")
  );

  axum::serve(listener, app).await.unwrap();
}
