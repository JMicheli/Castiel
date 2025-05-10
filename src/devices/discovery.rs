//! Defines functions for discovering Chromecast devices on the network and returning information to the frontend.

use std::{
  collections::HashMap,
  time::{Duration, Instant},
};

use flume::RecvTimeoutError;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use serde::Serialize;

use crate::errors::CastielError;

/// Used to inform the mdns browse command on what services are being searched for.
const SERVICE_TYPE: &str = "_googlecast._tcp.local.";

#[derive(Debug, Serialize)]
pub struct DiscoveredDevice {
  /// IPv4 or IPv6 address of the discovered Chromecast device.
  pub ip_address: String,

  /// Port on which the Chromecast service is listening (typically 8009).
  pub port: u16,

  /// Full mDNS service name, e.g. "Chromecast-<id>._googlecast._tcp.local."
  pub fullname: String,

  /// Chromecast device ID (TXT key "id")
  pub id: Option<String>,

  /// Model name (TXT key "md")
  pub model_name: Option<String>,

  /// Friendly name (TXT key "fn")
  pub friendly_name: Option<String>,

  /// All raw TXT properties
  pub txt_properties: HashMap<String, String>,
}

impl TryFrom<ServiceInfo> for DiscoveredDevice {
  type Error = CastielError;

  fn try_from(info: ServiceInfo) -> Result<Self, Self::Error> {
    // Grab the first ip address (error if none found)
    let ip_address = info
      .get_addresses()
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<_>>()
      .first()
      .ok_or(CastielError::InternalError)?
      .clone();

    // Grab the TXT record struct
    let props = info.get_properties();

    // Pull out standard Chromecast keys
    let id = props.get_property_val_str("id").map(ToString::to_string);
    let model_name = props.get_property_val_str("md").map(ToString::to_string);
    let friendly_name = props.get_property_val_str("fn").map(ToString::to_string);

    // Collect TXT properties into a map
    let mut txt_properties = HashMap::new();
    for prop in props.iter() {
      txt_properties.insert(prop.key().to_string(), prop.val_str().to_string());
    }

    Ok(Self {
      ip_address,
      port: info.get_port(),
      fullname: info.get_fullname().to_string(),
      id,
      model_name,
      friendly_name,
      txt_properties,
    })
  }
}

/// Search for chromecasts for as long as the `search_seconds` parameter asks.
pub fn find_chromecasts(search_seconds: u64) -> Result<Vec<DiscoveredDevice>, CastielError> {
  // Create daemon and receiver
  let mdns = ServiceDaemon::new().map_err(|_| CastielError::InternalError)?;
  let receiver = mdns
    .browse(SERVICE_TYPE)
    .map_err(|_| CastielError::InternalError)?;

  // Create HashMap to store viewed chromecasts and avoid duplication
  let mut seen = HashMap::new();

  // Listen for events until timeout elapses
  let search_timeout = Duration::from_secs(search_seconds);
  let start = Instant::now();
  while Instant::now().duration_since(start) < search_timeout {
    // TODO - Do I want a 500 ms timeout here?
    match receiver.recv_timeout(Duration::from_millis(500)) {
      Ok(ServiceEvent::ServiceResolved(info)) => {
        // Try creating a device and adding it to the HashMap.
        // This fails if the ServiceInfo has no ip addresses.
        if let Ok(device) = DiscoveredDevice::try_from(info) {
          seen.entry(device.fullname.clone()).or_insert(device);
        }
      }
      Ok(_) | Err(RecvTimeoutError::Timeout) => { /* Continue looping until global timeout */ }
      Err(err) => {
        tracing::error!("mDNS receive error: {err}");
        return Err(CastielError::InternalError);
      }
    }
  }

  // Stop service daemon and return list
  let _ = mdns.stop_browse(SERVICE_TYPE);
  Ok(seen.into_values().collect())
}
