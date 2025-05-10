//! Defines an error enum for use throughout Boardcast.
//!
//! The error can be transformed into an HTTP response and is used in all
//! API outputs.

use axum::{http::StatusCode, response::IntoResponse, response::Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BCError {
  #[error("Configuration error: {0}")]
  ConfigError(#[from] config::ConfigError),
  #[error("I/O error: {0}")]
  IoError(#[from] std::io::Error),
  #[error("JSON error: {0}")]
  JsonError(#[from] serde_json::Error),
  #[error("Chromecast media error: {0}")]
  MediaError(rust_cast::errors::Error),
  #[error("Chromecast app error: {0}")]
  AppError(rust_cast::errors::Error),
  #[error("Chromecast connection error: {0}")]
  ConnError(rust_cast::errors::Error),
  #[error("Chromecast device lookup failed: {0}")]
  DeviceLookupFailed(rust_cast::errors::Error),
  #[error("Chromecast app lookup failed")]
  AppLookupFailed,
  #[error("Internal server error")]
  InternalError,
}

impl IntoResponse for BCError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      Self::ConfigError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      Self::IoError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      Self::JsonError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      Self::MediaError(err) | Self::AppError(err) => {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
      }
      Self::ConnError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      Self::DeviceLookupFailed(err) => (StatusCode::NOT_FOUND, err.to_string()),
      Self::AppLookupFailed => (StatusCode::NOT_FOUND, Self::AppLookupFailed.to_string()),
      Self::InternalError => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Self::InternalError.to_string(),
      ),
    };

    (status, error_message).into_response()
  }
}
