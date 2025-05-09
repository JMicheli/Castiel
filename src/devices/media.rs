//! Defines functionality for starting media playback and display on Chromecast devices.

use rust_cast::{
  CastDevice,
  channels::{
    media::{Media, StreamType},
    receiver::CastDeviceApp,
  },
};
use serde::{Deserialize, Serialize};

const DEFAULT_DESTINATION_ID: &str = "receiver-0";

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
pub fn start_from_data(data: StartMediaData) -> Result<(), rust_cast::errors::Error> {
  tracing::info!("Starting media from data: {data:?}");

  let ip = data.ip_address.clone();
  let cast_device = get_cast_device(&ip, data.port)?;

  match data.receiver {
    ReceiverOptions::Default => {
      start_app_and_media(cast_device, CastDeviceApp::DefaultMediaReceiver, data)?
    }
    ReceiverOptions::YouTube => start_app_and_media(cast_device, CastDeviceApp::YouTube, data)?,
    ReceiverOptions::Web => start_web_media(cast_device, data.media_url)?,
  }

  Ok(())
}

/// Retrieve a cast device by `ip` and `port`.
fn get_cast_device(ip: &str, port: u16) -> Result<CastDevice, rust_cast::errors::Error> {
  // TODO - Figure out how to use host verification properly.
  let cast_device = CastDevice::connect_without_host_verification(ip, port)?;

  // Test connection
  // TODO - Make sure I understand what this is doing.
  cast_device
    .connection
    .connect(DEFAULT_DESTINATION_ID.to_string())?;
  cast_device.heartbeat.ping()?;

  Ok(cast_device)
}

/// Uses the `cast_device` to start the specified `app_to_start` and begin playing `data`.
///
/// This function can be used to start generic media with [`CastDeviceApp::DefaultMediaReciever`]
/// or YouTube videos with [`CastDevice::YouTube`].
fn start_app_and_media(
  cast_device: CastDevice,
  app_to_start: CastDeviceApp,
  data: StartMediaData,
) -> Result<(), rust_cast::errors::Error> {
  let app = cast_device.receiver.launch_app(&app_to_start)?;

  cast_device.connection.connect(app.transport_id.as_str())?;

  let media = Media {
    content_id: data.media_url,
    content_type: data.content_type,
    stream_type: data.stream_type.into(),
    duration: None,
    metadata: None,
  };

  cast_device
    .media
    .load(app.transport_id, app.session_id, &media)?;

  Ok(())
}

/// The ID for the web page viewing app.
///
/// This points to an app created for this project:
/// https://github.com/davestevens/chromecast-webpage-viewer
const WEB_APP_ID: &str = "209991B4";
/// The namespace for messages to the web viewing app.
const WEB_APP_NAMESPACE: &str = "urn:x-cast:uk.co.ecksdee";

#[derive(Debug, Serialize)]
struct WebAppMessage {
  url: String,
  proxy: bool,
}

fn start_web_media(
  cast_device: CastDevice,
  media_url: String,
) -> Result<(), rust_cast::errors::Error> {
  // Launch web viewer app
  let app_to_launch = CastDeviceApp::Custom(WEB_APP_ID.to_string());
  let app = cast_device.receiver.launch_app(&app_to_launch)?;

  // Start connection to web viewer
  cast_device.connection.connect(app.transport_id.as_str())?;

  // Broadcast a message to the running web viewer app
  cast_device.receiver.broadcast_message(
    WEB_APP_NAMESPACE,
    &WebAppMessage {
      url: media_url,
      proxy: false,
    },
  )?;

  Ok(())
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
