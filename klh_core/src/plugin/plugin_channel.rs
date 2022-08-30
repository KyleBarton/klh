use log::{info, debug};
use tokio::sync::mpsc;

use crate::messaging::{Message, MessageType};

use super::Plugin;

pub(crate) struct PluginChannel {
  listener: PluginListener,
  transmitter: PluginTransmitter,
  // Long term: Figure out how to encapsulate Send
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
	message_types: plugin.list_message_types(),
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

  pub(crate) fn get_transmitter(&self) -> Result<PluginTransmitter, String> {
    Ok(self.transmitter.clone())
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
  message_types: Vec<MessageType>,
  transmitter: mpsc::Sender<Message>,
}

impl PluginTransmitter {
  
  pub(crate) async fn send_message(&self, message: Message) -> Result<(), mpsc::error::SendError<Message>> {
    self.transmitter.send(message).await
  }


  // TODO can transmitter not own this? Should it really own the message
  // types?
  pub(crate) fn get_message_types(&self) -> Vec<MessageType> {
    self.message_types.clone()
  }
}
