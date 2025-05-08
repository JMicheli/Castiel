use rust_cast::{
  CastDevice, ChannelMessage,
  channels::{
    heartbeat::HeartbeatResponse,
    media::{Media, StreamType},
    receiver::CastDeviceApp,
  },
};
use serde_json::json;

// const SERVICE_TYPE: &str = "_googlecast._tcp.local.";
const DEFAULT_DESTINATION_ID: &str = "receiver-0";

fn main() {
  println!("Starting experiment");

  let cast_device = match CastDevice::connect_without_host_verification("192.168.1.58", 8009) {
    Ok(cast_device) => cast_device,
    Err(err) => panic!("Could not establish connection with Cast Device: {:?}", err),
  };

  cast_device
    .connection
    .connect(DEFAULT_DESTINATION_ID.to_string())
    .unwrap();
  cast_device.heartbeat.ping().unwrap();

  start_app(cast_device, "209991B4".to_string());
  //   start_media(cast_device);
  println!("Experiment concluded");
}

fn start_app(cast_device: CastDevice<'_>, id: String) {
  let app_to_launch = CastDeviceApp::Custom(id);
  let app = cast_device.receiver.launch_app(&app_to_launch).unwrap();

  cast_device
    .connection
    .connect(app.transport_id.as_str())
    .unwrap();

  let namespace = "urn:x-cast:uk.co.ecksdee";
  let message = json!({
    "url": "https://kashmir.liveuamap.com/",
    "proxy": false
  });

  cast_device
    .receiver
    .broadcast_message(namespace, &message)
    .unwrap();

  loop {
    match cast_device.receive() {
      Ok(ChannelMessage::Heartbeat(response)) => {
        println!("[Heartbeat] {:?}", response);

        if let HeartbeatResponse::Ping = response {
          cast_device.heartbeat.pong().unwrap();
        }
      }

      Ok(ChannelMessage::Connection(response)) => println!("[Connection] {:?}", response),
      Ok(ChannelMessage::Media(response)) => println!("[Media] {:?}", response),
      Ok(ChannelMessage::Receiver(response)) => println!("[Receiver] {:?}", response),
      Ok(ChannelMessage::Raw(response)) => println!(
        "Support for the following message type is not yet supported: {:?}",
        response
      ),

      Err(error) => println!("Error occurred while receiving message {}", error),
    }
  }
}

fn start_media(cast_device: CastDevice<'_>) {
  //   let app_to_launch = CastDeviceApp::Custom("web-1".to_string());
  let app_to_launch = CastDeviceApp::DefaultMediaReceiver;
  let app = cast_device.receiver.launch_app(&app_to_launch).unwrap();

  cast_device
    .connection
    .connect(app.transport_id.as_str())
    .unwrap();

  let media = Media {
    content_id: "https://jmicheli.github.io/assets/images/author_photo.png?v=d4aa6beb9e4b69d0b1b49edfd720c55534fbd45d".to_string(),
    content_type: "image/png".to_string(),
    stream_type: StreamType::Buffered,
    duration: None,
    metadata: None,
  };

  cast_device
    .media
    .load(app.transport_id, app.session_id, &media)
    .unwrap();

  loop {
    match cast_device.receive() {
      Ok(ChannelMessage::Heartbeat(response)) => {
        println!("[Heartbeat] {:?}", response);

        if let HeartbeatResponse::Ping = response {
          cast_device.heartbeat.pong().unwrap();
        }
      }

      Ok(ChannelMessage::Connection(response)) => println!("[Connection] {:?}", response),
      Ok(ChannelMessage::Media(response)) => println!("[Media] {:?}", response),
      Ok(ChannelMessage::Receiver(response)) => println!("[Receiver] {:?}", response),
      Ok(ChannelMessage::Raw(response)) => println!(
        "Support for the following message type is not yet supported: {:?}",
        response
      ),

      Err(error) => println!("Error occurred while receiving message {}", error),
    }
  }
}
