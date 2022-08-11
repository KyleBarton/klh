use std::collections::HashMap;

use crate::event::{EventMessage, EventType};

use super::PluginChannel;
use super::plugin_channel::PluginTransmitter;


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
  pub(crate) fn register_plugin_event_types(&mut self, plugin_channel: &PluginChannel) -> Result<(), String> {
    let transmitter: PluginTransmitter = plugin_channel.get_transmitter().unwrap().clone();
    for event_type in transmitter.get_event_types().iter() {
      println!("Registering event type {}", event_type);
      self.plugin_type_map.insert(event_type.clone(), transmitter.clone());
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
