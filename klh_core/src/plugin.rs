// plugin.rs.

use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{event::Event, dispatch::DispatchClient};

#[derive(Clone)]
pub struct PluginListener {
  event_transmitter: mpsc::Sender<Event>
}

// TODO needs cleanup
impl PluginListener {
  
  async fn send_event(&self, event: Event) {
    self.event_transmitter.send(event).await.unwrap()
  }
}

pub trait Plugin {
  fn accept_event(&self) -> Result<(), String>;

  fn clone_listener(&self) -> Result<PluginListener, String>;

  fn list_events(&self) -> Vec<Event>;

  fn receive_client(&self, dispatch_client: DispatchClient);
}


#[derive(Clone)]
pub(crate) struct PluginRegistrar {
  plugins: HashMap<Event, PluginListener>,
}

impl PluginRegistrar {

  pub(crate) fn new() -> Self {
    PluginRegistrar {
      plugins: HashMap::new()
    }
  }

  pub(crate) fn register_plugin(&mut self, plugin: impl Plugin) -> Result<(), String> {
    for plugin_event in plugin.list_events().iter() {
      match plugin.clone_listener() {
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
