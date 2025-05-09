mod discovery;

use axum::{Json, Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
  let serve_dir =
    ServeDir::new("frontend/dist").not_found_service(ServeFile::new("frontend/dist/index.html"));

  let app = Router::new()
    .route("/api/chromecasts", get(get_chromecasts))
    .fallback_service(serve_dir);

  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
  println!("Listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}

/// Handler for the GET /api/chromecasts endpoint.
///
/// Runs device discovery and returns a list of discovered Chromecast devices as JSON.
async fn get_chromecasts() -> Result<Json<Vec<discovery::DiscoveredDevice>>, StatusCode> {
  // Search for Chromecasts for 2 seconds
  let devices = discovery::find_chromecasts(2);

  // Log the discovered devices
  println!(
    "Found {} Chromecast device(s): {:?}",
    devices.len(),
    devices
  );

  // Return the devices as JSON
  Ok(Json(devices))
}
