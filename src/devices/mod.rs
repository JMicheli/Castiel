pub mod app_ids;
pub mod discovery;
pub mod media;
pub mod status;

use rust_cast::CastDevice;

use crate::errors::BCError;

const DEFAULT_DESTINATION_ID: &str = "receiver-0";

/// Retrieve a cast device by `ip` and `port`.
fn get_cast_device(ip: &str, port: u16) -> Result<CastDevice, BCError> {
  // TODO - Figure out how to use host verification properly.
  let cast_device =
    CastDevice::connect_without_host_verification(ip, port).map_err(BCError::DeviceLookupFailed)?;

  // Test connection
  // TODO - Honestly, can I just remove this? Seems like no - it makes status requests fail.
  cast_device
    .connection
    .connect(DEFAULT_DESTINATION_ID.to_string())
    .map_err(BCError::ConnError)?;
  cast_device.heartbeat.ping().map_err(BCError::ConnError)?;

  Ok(cast_device)
}
