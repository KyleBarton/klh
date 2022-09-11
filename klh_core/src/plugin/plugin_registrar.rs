use std::collections::HashMap;

use log::{debug, warn};

use crate::messaging::{Message, MessageType, MessageContent, MessageError};

use super::plugin_channel::PluginTransmitter;


#[derive(Clone)]
pub(crate) struct PluginRegistrar {
  plugin_type_map: HashMap<MessageType, PluginTransmitter>,
}

impl PluginRegistrar {

  pub(crate) fn new() -> Self {
    PluginRegistrar {
      plugin_type_map: HashMap::new(),
    }
  }

  pub(crate) fn register_message_types_for_plugin(
    &mut self,
    message_types: Vec<MessageType>,
    transmitter: PluginTransmitter
  ) {
    for message_type in message_types.iter() {
      debug!("Registering message type {}", message_type);
      self.plugin_type_map.insert(message_type.clone(), transmitter.clone());
    }
  }

  pub(crate) async fn send_to_plugin(&self, mut message: Message) {
    debug!("Plugin registrar received message {}", message);
    match self.plugin_type_map.get(&message.get_message_type()) {
      Some(listener) => {
	debug!("Found listener for message {}", message);
	listener.send_message(message).await.unwrap();
      },
      None => {
	warn!("Could not find listener message {}", message);
	message.get_responder()
	  .expect("Should have a responder")
	  .respond(MessageContent::from_content(MessageError::MessageTypeNotFound))
	  .unwrap();
      },
    }
  }
}
