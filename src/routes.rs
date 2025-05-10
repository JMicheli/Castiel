//! Defines Boardcast's API routes and their handlers.

use axum::{
  Json, Router,
  routing::{get, post},
};
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};

use crate::{
  devices::{
    self,
    discovery::DiscoveredDevice,
    media::StartMediaData,
    status::{DeviceStatus, MediaStatus},
  },
  errors::BCError,
};

/// Creates the main application router.
pub fn create_router() -> Router {
  // Static file server for frontend.
  let serve_dir =
    ServeDir::new("frontend/dist").not_found_service(ServeFile::new("frontend/dist/index.html"));

  Router::new()
    .route("/api/chromecasts", get(get_chromecasts))
    .route("/api/start-media", post(send_media_handler))
    .route("/api/device-status", post(check_device_status))
    .route("/api/media-status", post(check_media_status))
    .fallback_service(serve_dir)
}

/// Handler for the GET /api/chromecasts endpoint.
///
/// Runs device discovery and returns a list of discovered Chromecast devices as JSON.
async fn get_chromecasts() -> Result<Json<Vec<DiscoveredDevice>>, BCError> {
  // Search for Chromecasts for 2 seconds
  let devices = devices::discovery::find_chromecasts(1)?;

  // Log the discovered devices
  let device_count = devices.len();
  let s = if device_count > 1 { "s" } else { "" };
  tracing::info!("Found {device_count} Chromecast device{s}.");
  tracing::trace!("Devices: {devices:?}");

  // Return the devices as JSON
  Ok(Json(devices))
}

/// Handler for the POST /api/send-media endpoint.
///
/// Receives media data from the frontend and initiates the media sending process.
async fn send_media_handler(Json(media_data): Json<StartMediaData>) -> Result<(), BCError> {
  devices::media::start_from_data(media_data)?;
  Ok(())
}

#[derive(Debug, Deserialize)]
/// A serialization structure for a device address sent in an API request.
pub struct DeviceAddress {
  pub ip: String,
  pub port: u16,
}

/// Handler for the GET /api/device-status endpoint.
///
/// Checks device status from the provided device address and returns it as JSON.
async fn check_device_status(
  Json(device_addr): Json<DeviceAddress>,
) -> Result<Json<DeviceStatus>, BCError> {
  let status = devices::status::get_device_status(&device_addr.ip, device_addr.port)?;
  Ok(Json(status))
}

async fn check_media_status(
  Json(device_addr): Json<DeviceAddress>,
) -> Result<Json<MediaStatus>, BCError> {
  let status = devices::status::get_media_status(&device_addr.ip, device_addr.port)?;
  Ok(Json(status))
}
