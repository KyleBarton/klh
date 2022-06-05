// plugin.rs.

use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{event::Event, dispatch::DispatchClient};

pub struct PluginChannel {
  pub listener: PluginListener,
  pub transmitter: PluginTransmitter,
}

impl PluginChannel {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel(128);
    Self {
      listener: PluginListener {
	event_listener: rx
      },
      transmitter: PluginTransmitter {
	event_transmitter: tx
      },
    }
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
  event_transmitter: mpsc::Sender<Event>,
}

// TODO needs cleanup
impl PluginTransmitter {
  
  async fn send_event(&self, event: Event) {
    self.event_transmitter.send(event).await.unwrap()
  }
}

pub trait Plugin {

  fn accept_event(&self, event: Event) -> Result<(), String>;
  
  fn clone_transmitter(&self) -> Result<PluginTransmitter, String>;

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

  pub(crate) fn register_plugin(&mut self, plugin: impl Plugin) -> Result<(), String> {
    for plugin_event in plugin.list_events().iter() {
      match plugin.clone_transmitter() {
	Ok(listener) => self.plugins.insert(Event::from(plugin_event), listener),
	Err(_) => return Err(String::from("Something went wrong cloning the plugin listener"))
      };
    }

    Ok(())
  }

  pub(crate) async fn send_to_plugin(&self, event: Event) {
    match self.plugins.get(&event) {
      Some(listener) => listener.send_event(Event::from(&event)).await,
      None => (),
    }
  }
}
