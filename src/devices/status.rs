//! Defines functionality for checking the status of Chromecast devices.

use serde::Serialize;

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

pub fn get_device_status(ip: &str, port: u16) -> Result<DeviceStatus, rust_cast::errors::Error> {
  tracing::info!("Getting device status for {ip}:{port}");
  let cast_device = super::get_cast_device(ip, port)?;

  let status = cast_device.receiver.get_status()?;
  Ok(DeviceStatus::from(status))
}
