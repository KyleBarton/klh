use log::{info, debug};
use tokio::sync::mpsc;

use crate::messaging::Message;

use super::Plugin;

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

  pub(crate) async fn start(&mut self) {
    while let Some(message) = self.listener.receive().await {
      debug!("Received message on PluginChannel: {}", message);
      self.plugin.accept_message(message).unwrap();
      
    }
    info!("Plugin stopped listening");
  }

  pub(crate) fn get_transmitter(&self) -> PluginTransmitter {
    self.transmitter.clone()
  }
}

struct PluginListener {
  listener: mpsc::Receiver<Message>,
}

impl PluginListener {
  async fn receive(&mut self) -> Option<Message> {
    self.listener.recv().await
  }
}

#[derive(Clone)]
pub(crate) struct PluginTransmitter {
  transmitter: mpsc::Sender<Message>,
}

impl PluginTransmitter {
  
  pub(crate) async fn send_message(&self, message: Message) -> Result<(), mpsc::error::SendError<Message>> {
    self.transmitter.send(message).await
  }
}
