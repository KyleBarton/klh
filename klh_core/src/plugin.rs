use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{event::{EventType, EventMessage}, session::SessionClient};


pub struct PluginChannel {
  pub listener: PluginListener,
  pub transmitter: PluginTransmitter,
  // Long term: Figure out how to encapsulate Send
  pub plugin: Box<dyn Plugin + Send>,
}

impl PluginChannel {
  pub fn new(plugin: Box<dyn Plugin + Send>) -> Self {
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

  pub async fn start(&mut self) {
    while let Some(event_message) = self.listener.receive().await {
      println!("Received event for plugin on the PluginChannel: {}", event_message);
      self.plugin.accept_event(event_message).unwrap();
      
    }
    println!("Plugin stopped listening");
  }

  pub fn get_transmitter(&self) -> Result<PluginTransmitter, String> {
    Ok(self.transmitter.clone())
  }
}

pub struct PluginListener {
  event_listener: mpsc::Receiver<EventMessage>,
}

impl PluginListener {
  pub async fn receive(&mut self) -> Option<EventMessage> {
    self.event_listener.recv().await
  }
}

#[derive(Clone)]
pub struct PluginTransmitter {
  event_types: Vec<EventType>,
  event_transmitter: mpsc::Sender<EventMessage>,
}

impl PluginTransmitter {
  
  async fn send_event(&self, event_message: EventMessage) -> Result<(), mpsc::error::SendError<EventMessage>> {
    self.event_transmitter.send(event_message).await
  }


  // TODO can transmitter not own this? Should it really own the event
  // types?
  fn get_event_types(&self) -> Vec<EventType> {
    self.event_types.clone()
  }
}

pub trait Plugin {

  fn accept_event(&mut self, event_message: EventMessage) -> Result<(), String>;

  fn list_event_types(&self) -> Vec<EventType>;

  fn receive_client(&mut self, client: SessionClient);

}


#[derive(Clone)]
pub(crate) struct PluginRegistrar {
  plugin_type_map: HashMap<EventType, PluginTransmitter>,
}

impl PluginRegistrar {

  pub(crate) fn new() -> Self {
    PluginRegistrar {
      plugin_type_map: HashMap::new(),
    }
  }

  // Registers all event types which should be associated with a given plugin.
  pub(crate) fn register_plugin_event_types(&mut self, plugin_transmitter: PluginTransmitter) -> Result<(), String> {
    for event_type in plugin_transmitter.get_event_types().iter() {
      println!("Registering event type {}", event_type);
      self.plugin_type_map.insert(event_type.clone(), plugin_transmitter.clone());
    }
    Ok(())
  }

  pub(crate) async fn send_to_plugin(&self, event_message: EventMessage) {
    println!("Trying to find event type {}", event_message.get_event_type());
    match self.plugin_type_map.get(&event_message.get_event_type()) {
      Some(listener) => {
	println!("Found plugin for event type, sending along");
	listener.send_event(event_message).await.unwrap();
      },
      None => {
	println!("Could not find a plugin for this event: {}", event_message.get_event_type());
	()
      },
    }
  }
}
