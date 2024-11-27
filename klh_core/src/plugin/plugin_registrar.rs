use std::collections::HashMap;

use log::{debug, warn};

use crate::messaging::{EventMessage, Message, MessageContent, MessageError, MessageType};

use super::plugin_channel::PluginTransmitter;


/// The repository of active plugins that a KLH instance manages
/// during its running session. Not part of the KLH public API.
#[derive(Clone)]
pub(crate) struct PluginRegistrar {
  command_plugin_map: HashMap<MessageType, PluginTransmitter>,
  event_plugin_map: HashMap<MessageType, Vec<PluginTransmitter>>,
}

impl PluginRegistrar {

  pub(crate) fn new() -> Self {
    PluginRegistrar {
      command_plugin_map: HashMap::new(),
      // TODO this isn't quite right, need eventmessage instead of message (no responder, true pub sub)
      event_plugin_map: HashMap::new(),
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
      match message_type {
        MessageType::Command(_) => {
	  self.command_plugin_map.insert(*message_type, transmitter.clone());
	},
        MessageType::Query(_) => {
	  self.command_plugin_map.insert(*message_type, transmitter.clone());
	},
        MessageType::Event(_) => {
	  match self.event_plugin_map.get_mut(message_type) {
	    Some(transmitters) => {
	      transmitters.push(transmitter.clone());
	    },
	    None => {
	      let transmitters = vec!(transmitter.clone());
	      self.event_plugin_map.insert(*message_type, transmitters);
	    },
	  }
	},
      };
    }
  }

  /// Receives an event message and sends it to all the transmitters
  /// subscribed.
  pub(crate) async fn send_event_to_plugin(&self, mut message: EventMessage) {
    debug!("Plugin registrar received event message {:?}", message);
    match self.event_plugin_map.get(&message.get_message_type()) {
      Some(listeners) => {
	for _listener in listeners {
	  // listener.send_message()
	  debug!("TODO sending message to listener");
	}
      },
      None => {
	debug!("No listeners found for event message {:?}", message)
      }
    }
  }

  /// Receives a Message and sends it to one of the PluginTransmitter
  /// instances stored in its plugin_type_map, based on the
  /// MessageType of the sent Message.
  pub(crate) async fn send_to_plugin(&self, mut message: Message) {
    debug!("Plugin registrar received message {}", message);
    match self.command_plugin_map.get(&message.get_message_type()) {
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
	if let Err(message_error) = message.get_responder()
	  .expect("Should have a responder")
	  .respond(MessageContent::from_content(MessageError::MessageTypeNotFound)) {
	    debug!(
	      "Received a message error attempting to respond with MessageTypeNotFound: {:?}",
	      message_error
	    );
	  }
      },
    }
  }
}
