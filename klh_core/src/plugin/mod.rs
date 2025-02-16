#[allow(clippy::module_inception)]
mod plugin;
pub use plugin::Plugin;

mod plugin_registrar;
pub(crate) use plugin_registrar::PluginRegistrar;

mod plugin_channel;
pub(crate) use plugin_channel::PluginChannel;


#[cfg(test)]
pub mod plugin_test_utility {
  pub const COMMAND_ID: &str = "commandId";
  pub const COMMAND_RESPONSE: &str = "commandResponse";

  use crate::messaging::{Message, MessageType, MessageContent, MessageError};
  use crate::session::SessionClient;
  use super::Plugin;
  
  pub struct TestPlugin {
    command_sent: bool,
  }

  impl Default for TestPlugin {
    fn default() -> Self {
      Self::new()
    }
  }

  impl TestPlugin {
    pub fn new() -> Self {
      Self {
	command_sent: false,
      }
    }
  }

  impl Plugin for TestPlugin {
    fn accept_message(&mut self, mut message: Message) -> Result<(), MessageError> {
      match message.get_message_type() {
	MessageType::Command(id) => {
	  if COMMAND_ID.as_bytes() == &id[0..COMMAND_ID.len()] {
	    self.command_sent = true;
	    message.get_responder()
	      .expect("Should be a command")
	      .expect("Should have responder")
	      .respond(MessageContent::from_content(COMMAND_RESPONSE.to_string()))
	      .unwrap();
	  }
	},
	// TODO a responder shouldn't occur for an event, so need some other way to handle this here.
	MessageType::Event(_) => ()
      };

      Ok(())
    }

    fn list_message_types(&self) -> Vec<MessageType> {
      vec!(
	MessageType::command_from_str(COMMAND_ID).unwrap(),
      )
    }

    fn receive_client(&mut self, _client: SessionClient) {
      
    }
  }
}
