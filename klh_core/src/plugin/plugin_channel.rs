use log::{info, debug};
use tokio::sync::mpsc;

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
  
  pub(crate) async fn send_message(&self, message: Message) -> Result<(), mpsc::error::SendError<Message>> {
    self.transmitter.send(message).await
  }
}
