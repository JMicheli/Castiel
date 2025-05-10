//! Defines functionality for checking the status of Chromecast devices.

use serde::Serialize;

use crate::{
  devices::app_ids::{BACKDROP_ID, DEFAULT_MEDIA_ID, WEBVIEW_ID, YOUTUBE_ID},
  errors::BCError,
};

/// The status of a chromecast device. Contains information about the current
/// volume, running app, and state of the device.
#[derive(Debug, Serialize)]
pub struct DeviceStatus {
  // TODO - Figure out wtf this means
  is_active_input: bool,
  // TODO - Figure this one out too
  in_standby: bool,
  /// The current volume settings of the device.
  volume: VolumeStatus,
  // TODO - Verify that it's okay to use Option instead of Vec.
  /// The status of the currently running application on the Chromecast.
  /// Can be [`None`] if no application is running, but this is not expected.
  app_status: Option<AppStatus>,
}

/// Contains the volume state of a device.
#[derive(Debug, Serialize)]
pub struct VolumeStatus {
  /// The current volume of the device from `0.0` to `1.0`.
  /// [`rust_cast`] can return a [`None`] volume, which defaults to `0.0`.
  volume: f32,
  /// Whether the device is muted.
  /// [`rust_cast`] can return [`None`], which defaults to `false`.
  muted: bool,
}

/// Contains the status of a running application on the device.
#[derive(Debug, Serialize)]
pub struct AppStatus {
  /// The raw `app_id` field reported by the device.
  id: String,
  /// The parsed `app_id`.
  /// This value will be [`ParsedApp:Unknown`] if not recognized.
  app_identity: ParsedApp,
  /// The display name of the application.
  diplay_name: String,
  /// The namespaces used by the application.
  namespaces: Vec<String>,
  /// The session id of the application.
  session_id: String,
  /// A string representing the status of the application.
  status: String,
  /// The transport id for the application's receiver.
  transport_id: String,
}

#[derive(Debug, Serialize)]
pub enum ParsedApp {
  Backdrop,
  DefaultMedia,
  YouTube,
  WebView,
  Unknown,
}

impl From<rust_cast::channels::receiver::Status> for DeviceStatus {
  fn from(status: rust_cast::channels::receiver::Status) -> Self {
    let app_status = status.applications.into_iter().next().map(AppStatus::from);

    Self {
      is_active_input: status.is_active_input,
      in_standby: status.is_stand_by,
      // TODO - Make sure I can get away with this default unwrap strategy
      volume: VolumeStatus {
        volume: status.volume.level.unwrap_or(0.0),
        muted: status.volume.muted.unwrap_or(false),
      },
      app_status,
    }
  }
}

impl From<rust_cast::channels::receiver::Application> for AppStatus {
  fn from(app: rust_cast::channels::receiver::Application) -> Self {
    let app_identity = ParsedApp::from(app.app_id.as_str());

    Self {
      id: app.app_id,
      app_identity,
      diplay_name: app.display_name,
      namespaces: app.namespaces,
      session_id: app.session_id,
      status: app.status_text,
      transport_id: app.transport_id,
    }
  }
}

impl From<&str> for ParsedApp {
  fn from(id: &str) -> Self {
    match id {
      BACKDROP_ID => Self::Backdrop,
      DEFAULT_MEDIA_ID => Self::DefaultMedia,
      YOUTUBE_ID => Self::YouTube,
      WEBVIEW_ID => Self::WebView,
      _ => Self::Unknown,
    }
  }
}

pub fn get_device_status(ip: &str, port: u16) -> Result<DeviceStatus, BCError> {
  tracing::info!("Getting device status for {ip}:{port}");
  let cast_device = super::get_cast_device(ip, port)?;

  let device_status = cast_device
    .receiver
    .get_status()
    .map_err(BCError::ConnError)?;
  Ok(DeviceStatus::from(device_status))
}

#[derive(Debug, Serialize)]
pub struct MediaStatus {
  current_time: Option<f32>,
  playback_rate: f32,
  player_state: PlayerState,
}

#[derive(Debug, Serialize)]
pub enum PlayerState {
  Idle,
  Playing,
  Buffering,
  Paused,
}

impl Default for MediaStatus {
  fn default() -> Self {
    Self {
      current_time: Option::default(),
      playback_rate: 0.0,
      player_state: PlayerState::Idle,
    }
  }
}

impl From<rust_cast::channels::media::Status> for MediaStatus {
  fn from(status: rust_cast::channels::media::Status) -> Self {
    status
      .entries
      .first()
      .map_or_else(Self::default, |entry| Self {
        current_time: entry.current_time,
        playback_rate: entry.playback_rate,
        player_state: entry.player_state.into(),
      })
  }
}

impl From<rust_cast::channels::media::PlayerState> for PlayerState {
  fn from(state: rust_cast::channels::media::PlayerState) -> Self {
    match state {
      rust_cast::channels::media::PlayerState::Idle => Self::Idle,
      rust_cast::channels::media::PlayerState::Playing => Self::Playing,
      rust_cast::channels::media::PlayerState::Buffering => Self::Buffering,
      rust_cast::channels::media::PlayerState::Paused => Self::Paused,
    }
  }
}

pub fn get_media_status(ip: &str, port: u16) -> Result<MediaStatus, BCError> {
  tracing::info!("Getting media status for {ip}:{port}");
  let cast_device = super::get_cast_device(ip, port)?;

  let device_status = cast_device
    .receiver
    .get_status()
    .map_err(BCError::ConnError)?;
  let app = device_status
    .applications
    .first()
    .ok_or(BCError::AppLookupFailed)?;

  let media_status = cast_device
    .media
    .get_status(&app.transport_id, None)
    .map_err(BCError::ConnError)?;

  Ok(MediaStatus::from(media_status))
}
