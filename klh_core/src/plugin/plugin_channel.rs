use log::{info, debug};
use tokio::sync::mpsc::{self, error::SendError};

use crate::messaging::Message;

use super::Plugin;

/// The struct by which KLH creates asynchronous channels for Plugins
/// to listen for messages. This is not part of the public KLH API,
/// and is instead private integrating code that allows KLH to
/// properly isolate Plugins from each other.
pub(crate) struct PluginChannel {
  listener: PluginListener,
  transmitter: PluginTransmitter,
  plugin: Box<dyn Plugin + Send>,
}

impl PluginChannel {
  pub(crate) fn new(plugin: Box<dyn Plugin + Send>) -> Self {
    let (tx, rx) = mpsc::channel(128);
    Self {
      listener: PluginListener {
	listener: rx,
      },
      transmitter: PluginTransmitter {
	transmitter: tx,
      },
      plugin,
    }
  }

  /// Start listening for messages to sent along to the Plugin.
  pub(crate) async fn start(&mut self) {
    while let Some(message) = self.listener.receive().await {
      debug!("Received message on PluginChannel: {}", message);
      self.plugin.accept_message(message).unwrap();
      
    }
    info!("Plugin stopped listening");
  }

  /// Provide a transmitter by which messages can be sent along this
  /// PluginChannel. Typically this is called by the PluginRegistrar
  /// of the KLH instance, which keeps a copy of a PluginTransmitter
  /// for each registered Plugin.
  pub(crate) fn get_transmitter(&self) -> PluginTransmitter {
    self.transmitter.clone()
  }
}

/// Wrapper for receiving messages along the PluginChannel.
struct PluginListener {
  listener: mpsc::Receiver<Message>,
}

impl PluginListener {
  async fn receive(&mut self) -> Option<Message> {
    self.listener.recv().await
  }
}

/// Wrapper for sending messages along the PluginChannel
#[derive(Clone)]
pub(crate) struct PluginTransmitter {
  transmitter: mpsc::Sender<Message>,
}

impl PluginTransmitter {
  
  pub(crate) async fn send_message(&self, message: Message) -> Result<(), SendError<Message>> {
    self.transmitter.send(message).await
  }
}

#[cfg(test)]
mod plugin_channel_tests {
  use rstest::*;

  use crate::plugin::plugin_test_utility::{TestPlugin, QUERY_RESPONSE, QUERY_ID};
  use crate::messaging::{Request, MessageType, Message, MessageContent};

  use super::PluginChannel;

  #[fixture]
  fn message_to_send() -> Message {
    Request::from_message_type(
      MessageType::query_from_str(QUERY_ID).unwrap()
    ).to_message()
  }

  #[rstest]
  #[tokio::test]
  async fn should_send_message_through_plugin_channel() {
    let mut given_request = Request::from_message_type(
      MessageType::query_from_str(QUERY_ID).unwrap()
    );
    let mut response_handler = given_request.get_handler()
      .expect("response handler should be available");

    let message = given_request.to_message();

    let mut plugin_channel : PluginChannel = PluginChannel::new(
      Box::new(TestPlugin::new()),
    );

    let transmitter = plugin_channel.get_transmitter();


    tokio::spawn(async move {
      plugin_channel.start().await;
    });

    let _ = transmitter.send_message(message).await;

    let response = response_handler.handle_response().await.unwrap();

    assert_eq!(response, MessageContent::from_content(QUERY_RESPONSE))
  }
}
