//! Defines functionality for checking the status of Chromecast devices.

use serde::Serialize;

use crate::errors::BCError;

#[derive(Debug, Serialize)]
pub struct DeviceStatus {
  is_active_input: bool,
  in_standby: bool,
  volume: VolumeStatus,
  app_status: Vec<AppStatus>,
}

#[derive(Debug, Serialize)]
pub struct VolumeStatus {
  volume: f32,
  muted: bool,
}

#[derive(Debug, Serialize)]
pub struct AppStatus {
  id: String,
  diplay_name: String,
  namespaces: Vec<String>,
  session_id: String,
  status: String,
  transport_id: String,
}

impl From<rust_cast::channels::receiver::Status> for DeviceStatus {
  fn from(status: rust_cast::channels::receiver::Status) -> Self {
    let app_status = status
      .applications
      .into_iter()
      .map(|app| AppStatus {
        id: app.app_id,
        diplay_name: app.display_name,
        namespaces: app.namespaces,
        session_id: app.session_id,
        status: app.status_text,
        transport_id: app.transport_id,
      })
      .collect();

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
      current_time: Default::default(),
      playback_rate: 0.0,
      player_state: PlayerState::Idle,
    }
  }
}

impl From<rust_cast::channels::media::Status> for MediaStatus {
  fn from(status: rust_cast::channels::media::Status) -> Self {
    match status.entries.first() {
      Some(entry) => Self {
        current_time: entry.current_time,
        playback_rate: entry.playback_rate,
        player_state: entry.player_state.into(),
      },
      // TODO - Make sure we're okay with doing it this way
      None => MediaStatus::default(),
    }
  }
}

impl From<rust_cast::channels::media::PlayerState> for PlayerState {
  fn from(state: rust_cast::channels::media::PlayerState) -> Self {
    match state {
      rust_cast::channels::media::PlayerState::Idle => PlayerState::Idle,
      rust_cast::channels::media::PlayerState::Playing => PlayerState::Playing,
      rust_cast::channels::media::PlayerState::Buffering => PlayerState::Buffering,
      rust_cast::channels::media::PlayerState::Paused => PlayerState::Paused,
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
