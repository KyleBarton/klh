use tokio::sync::mpsc;

use crate::event::{EventMessage, EventType};

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
	event_listener: rx,
      },
      transmitter: PluginTransmitter {
	event_transmitter: tx,
	event_types: plugin.list_event_types(),
      },
      plugin,
    }
  }

  pub(crate) async fn start(&mut self) {
    while let Some(event_message) = self.listener.receive().await {
      println!("Received event for plugin on the PluginChannel: {}", event_message);
      self.plugin.accept_event(event_message).unwrap();
      
    }
    println!("Plugin stopped listening");
  }

  pub(crate) fn get_transmitter(&self) -> Result<PluginTransmitter, String> {
    Ok(self.transmitter.clone())
  }
}

struct PluginListener {
  event_listener: mpsc::Receiver<EventMessage>,
}

impl PluginListener {
  async fn receive(&mut self) -> Option<EventMessage> {
    self.event_listener.recv().await
  }
}

#[derive(Clone)]
pub(crate) struct PluginTransmitter {
  event_types: Vec<EventType>,
  event_transmitter: mpsc::Sender<EventMessage>,
}

impl PluginTransmitter {
  
  pub(crate) async fn send_event(&self, event_message: EventMessage) -> Result<(), mpsc::error::SendError<EventMessage>> {
    self.event_transmitter.send(event_message).await
  }


  // TODO can transmitter not own this? Should it really own the event
  // types?
  pub(crate) fn get_event_types(&self) -> Vec<EventType> {
    self.event_types.clone()
  }
}
