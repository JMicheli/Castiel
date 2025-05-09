//! Defines Boardcast's API routes and their handlers.

use axum::{
  Json, Router,
  http::StatusCode,
  routing::{get, post},
};
use tower_http::services::{ServeDir, ServeFile};

use crate::devices::{
  discovery,
  media::{self, StartMediaData},
};

/// Creates the main application router.
pub fn create_router() -> Router {
  // Static file server for frontend.
  let serve_dir =
    ServeDir::new("frontend/dist").not_found_service(ServeFile::new("frontend/dist/index.html"));

  Router::new()
    .route("/api/chromecasts", get(get_chromecasts))
    .route("/api/start-media", post(send_media_handler))
    .fallback_service(serve_dir)
}

/// Handler for the GET /api/chromecasts endpoint.
///
/// Runs device discovery and returns a list of discovered Chromecast devices as JSON.
async fn get_chromecasts() -> Result<Json<Vec<discovery::DiscoveredDevice>>, StatusCode> {
  // Search for Chromecasts for 2 seconds
  let devices = discovery::find_chromecasts(1);

  // Log the discovered devices
  let device_count = devices.len();
  tracing::info!("Found {device_count} Chromecast device(s).");
  tracing::trace!("Devices: {devices:?}");

  // Return the devices as JSON
  Ok(Json(devices))
}

/// Handler for the POST /api/send-media endpoint.
///
/// Receives media data from the frontend and initiates the media sending process.
async fn send_media_handler(Json(media_data): Json<StartMediaData>) -> Result<(), StatusCode> {
  match media::start_from_data(media_data) {
    Ok(_) => Ok(()),
    Err(err) => {
      tracing::error!("Error starting media: {err}");
      // TODO: Provide more specific error status codes based on the error type
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
