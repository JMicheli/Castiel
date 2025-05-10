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
      BCError::ConfigError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      BCError::IoError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      BCError::JsonError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      BCError::MediaError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      BCError::AppError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      BCError::ConnError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
      BCError::DeviceLookupFailed(err) => (StatusCode::NOT_FOUND, err.to_string()),
      BCError::AppLookupFailed => (StatusCode::NOT_FOUND, BCError::AppLookupFailed.to_string()),
      BCError::InternalError => (
        StatusCode::INTERNAL_SERVER_ERROR,
        BCError::InternalError.to_string(),
      ),
    };

    (status, error_message).into_response()
  }
}
