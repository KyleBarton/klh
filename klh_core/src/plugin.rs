// plugin.rs.

use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{event::Event, dispatch::DispatchClient};


// Plugin only handles sync logic. PluginChannel handles all async stuff.
pub struct PluginChannel {
  pub listener: PluginListener,
  pub transmitter: PluginTransmitter,
  // TODO Figure out how to encapsulate Send
  pub plugin: Box<dyn Plugin + Send>,
}

impl PluginChannel {
  pub fn new(plugin: Box<dyn Plugin + Send>) -> Self {
    let (tx, rx) = mpsc::channel(128);
    Self {
      listener: PluginListener {
	event_listener: rx
      },
      transmitter: PluginTransmitter {
	event_transmitter: tx,
	events: plugin.list_events(),
      },
      plugin,
    }
  }

  pub async fn start(&mut self) {
    while let Some(event) = self.listener.receive().await {
      println!("Received event for plugin on the PluginChannel: {:?}", event);
      // Because of the damn plugin!
      self.plugin.accept_event(event).unwrap();
      
    }
    println!("Plugin stopped listening");
    // Ok(())
  }

  pub fn get_transmitter(&self) -> Result<PluginTransmitter, String> {
    Ok(self.transmitter.clone())
  }
}

pub struct PluginListener {
  event_listener: mpsc::Receiver<Event>,
}

impl PluginListener {
  pub async fn receive(&mut self) -> Option<Event> {
    self.event_listener.recv().await
  }
}

#[derive(Clone)]
pub struct PluginTransmitter {
  events: Vec<Event>,
  event_transmitter: mpsc::Sender<Event>,
}

// TODO needs cleanup
impl PluginTransmitter {
  
  async fn send_event(&self, event: Event) -> Result<(), mpsc::error::SendError<Event>> {
    self.event_transmitter.send(event).await
  }

  fn get_events(&self) -> Vec<Event> {
    self.events.clone()
  }
}

pub trait Plugin {

  fn accept_event(&self, event: Event) -> Result<(), String>;
  
  fn list_events(&self) -> Vec<Event>;

  fn receive_client(&mut self, dispatch_client: DispatchClient);

}


#[derive(Clone)]
pub(crate) struct PluginRegistrar {
  plugins: HashMap<Event, PluginTransmitter>,
}

impl PluginRegistrar {

  pub(crate) fn new() -> Self {
    PluginRegistrar {
      plugins: HashMap::new()
    }
  }

  pub(crate) fn register_plugin_events(&mut self, plugin_transmitter: PluginTransmitter) -> Result<(), String> {
    for event in plugin_transmitter.get_events().iter() {
      println!("Registering event {:?}", event);
      self.plugins.insert(Event::from(event), plugin_transmitter.clone());
    }
    Ok(())
  }

  pub(crate) async fn send_to_plugin(&self, event: Event) {
    println!("Trying to find event {:?}", event);
    match self.plugins.get(&event) {
      Some(listener) => {
	listener.send_event(Event::from(&event)).await.unwrap();
      },
      None => {
	println!("Could not find a plugin for this event: {:?}", event);
	()
      },
    }
  }
}
