use rust_cast::CastDevice;

pub mod discovery;
pub mod media;
pub mod status;

const DEFAULT_DESTINATION_ID: &str = "receiver-0";

/// Retrieve a cast device by `ip` and `port`.
fn get_cast_device(ip: &str, port: u16) -> Result<CastDevice, rust_cast::errors::Error> {
  // TODO - Figure out how to use host verification properly.
  let cast_device = CastDevice::connect_without_host_verification(ip, port)?;

  // Test connection
  // TODO - Honestly, can I just remove this? Seems like no - it makes status requests fail.
  cast_device
    .connection
    .connect(DEFAULT_DESTINATION_ID.to_string())?;
  cast_device.heartbeat.ping()?;

  Ok(cast_device)
}
