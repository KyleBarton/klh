use std::collections::HashMap;

use log::{debug, warn};

use crate::messaging::{Message, MessageType, MessageContent, MessageError};

use super::plugin_channel::PluginTransmitter;


/// The repository of active plugins that a KLH instance manages
/// during its running session. Not part of the KLH public API.
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

  /// Receives a list of MessageTypes and PluginTransmitter instances,
  /// and creates a mapping from MessageType -> Transmitter in its
  /// plugin_type_map.
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

  /// Receives a Message and sends it to one of the PluginTransmitter
  /// instances stored in its plugin_type_map, based on the
  /// MessageType of the sent Message.
  pub(crate) async fn send_to_plugin(&self, mut message: Message) {
    debug!("Plugin registrar received message {}", message);
    match self.plugin_type_map.get(&message.get_message_type()) {
      Some(listener) => {
	debug!("Found listener for message {}", message);
	match listener.send_message(message).await {
	  Ok(_) => (),
	  Err(e) => {
	    debug!("Error sending message to plugin channel listener: {}", e);
	  },
	}
      },
      None => {
	warn!("Could not find listener message {}", message);
	match message.get_responder()
	  .expect("Should have a responder")
	  .respond(MessageContent::from_content(MessageError::MessageTypeNotFound)) {
	    Err(message_error) => {
	      debug!(
		"Received a message error attempting to respond with MessageTypeNotFound: {:?}",
		message_error
	      );
	    },
	    Ok(_) => (),
	  }
      },
    }
  }
}
