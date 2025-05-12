//! Defines functionality for starting media playback and display on Chromecast devices.

use std::str::FromStr;

use rust_cast::{
  CastDevice,
  channels::{
    media::{Media, StreamType},
    receiver::CastDeviceApp,
  },
};
use serde::{Deserialize, Serialize};

use crate::{
  devices::DeviceAddress,
  devices::app_ids::{BACKDROP_ID, WEBVIEW_ID, WEBVIEW_NAMESPACE},
  errors::CastielError,
};

#[derive(Debug, Deserialize)]
pub enum ReceiverOptions {
  Default,
  YouTube,
  Web,
}

#[derive(Debug, Deserialize)]
pub enum StreamTypeOptions {
  Live,
  Buffered,
  None,
}

impl From<StreamTypeOptions> for StreamType {
  fn from(opts: StreamTypeOptions) -> Self {
    match opts {
      StreamTypeOptions::Live => Self::Live,
      StreamTypeOptions::Buffered => Self::Buffered,
      StreamTypeOptions::None => Self::None,
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct StartMediaData {
  pub ip_address: String,
  pub port: u16,
  pub receiver: ReceiverOptions,
  pub media_url: String,
  pub content_type: String,
  pub stream_type: StreamTypeOptions,
}

/// Starts media using the contents of `StartMediaData`.
pub fn start_from_data(data: StartMediaData) -> Result<(), CastielError> {
  tracing::info!("Starting media from data: {data:?}");

  let ip = data.ip_address.clone();
  let cast_device = super::get_cast_device(&ip, data.port)?;

  match data.receiver {
    ReceiverOptions::Default => {
      start_app_and_media(&cast_device, &CastDeviceApp::DefaultMediaReceiver, data)?;
    }
    ReceiverOptions::YouTube => start_app_and_media(&cast_device, &CastDeviceApp::YouTube, data)?,
    ReceiverOptions::Web => start_web_media(&cast_device, data.media_url)?,
  }

  Ok(())
}

/// Uses the `cast_device` to start the specified `app_to_start` and begin playing `data`.
///
/// This function can be used to start generic media with [`CastDeviceApp::DefaultMediaReciever`]
/// or YouTube videos with [`CastDevice::YouTube`].
fn start_app_and_media(
  cast_device: &CastDevice,
  app_to_start: &CastDeviceApp,
  data: StartMediaData,
) -> Result<(), CastielError> {
  let app = cast_device
    .receiver
    .launch_app(app_to_start)
    .map_err(CastielError::AppError)?;

  cast_device
    .connection
    .connect(app.transport_id.as_str())
    .map_err(CastielError::ConnError)?;

  let media = Media {
    content_id: data.media_url,
    content_type: data.content_type,
    stream_type: data.stream_type.into(),
    duration: None,
    metadata: None,
  };

  cast_device
    .media
    .load(app.transport_id, app.session_id, &media)
    .map_err(CastielError::MediaError)?;

  Ok(())
}

#[derive(Debug, Serialize)]
struct WebAppMessage {
  url: String,
  proxy: bool,
}

fn start_web_media(cast_device: &CastDevice, media_url: String) -> Result<(), CastielError> {
  // Launch web viewer app
  let app_to_launch = CastDeviceApp::Custom(WEBVIEW_ID.to_string());
  let app = cast_device
    .receiver
    .launch_app(&app_to_launch)
    .map_err(CastielError::AppError)?;

  // Start connection to web viewer
  cast_device
    .connection
    .connect(app.transport_id.as_str())
    .map_err(CastielError::ConnError)?;

  // Broadcast a message to the running web viewer app
  cast_device
    .receiver
    .broadcast_message(
      WEBVIEW_NAMESPACE,
      &WebAppMessage {
        url: media_url,
        proxy: false,
      },
    )
    .map_err(CastielError::MediaError)?;

  Ok(())
}

pub fn stop_media_at_device(device_addr: DeviceAddress) -> Result<(), CastielError> {
  let cast_device = super::get_cast_device(&device_addr.ip, device_addr.port)?;

  // Get status
  let device_status = cast_device
    .receiver
    .get_status()
    .map_err(CastielError::ConnError)?;

  if let Some(app) = device_status.applications.first() {
    cast_device
      .receiver
      .stop_app(app.session_id.clone())
      .map_err(CastielError::AppError)?;
    Ok(())
  } else {
    Err(CastielError::AppLookupFailed)
  }
}

// TODO - Decide if there's anything to do with this
//
// loop {
//   match cast_device.receive() {
//     Ok(ChannelMessage::Heartbeat(response)) => {
//       println!("[Heartbeat] {:?}", response);
//
//       if let HeartbeatResponse::Ping = response {
//         cast_device.heartbeat.pong().unwrap();
//       }
//     }
//
//     Ok(ChannelMessage::Connection(response)) => println!("[Connection] {:?}", response),
//     Ok(ChannelMessage::Media(response)) => println!("[Media] {:?}", response),
//     Ok(ChannelMessage::Receiver(response)) => println!("[Receiver] {:?}", response),
//     Ok(ChannelMessage::Raw(response)) => println!(
//       "Support for the following message type is not yet supported: {:?}",
//       response
//     ),
//
//     Err(error) => println!("Error occurred while receiving message {}", error),
//   }
// }
