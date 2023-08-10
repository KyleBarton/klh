mod plugin;
pub use plugin::Plugin;

mod plugin_registrar;
pub(crate) use plugin_registrar::PluginRegistrar;

mod plugin_channel;
pub(crate) use plugin_channel::PluginChannel;


#[cfg(test)]
pub mod plugin_test_utility {
  pub const QUERY_ID: &str = "queryId";
  pub const COMMAND_ID: &str = "commandId";
  pub const COMMAND_RESPONSE: &str = "commandResponse";
  pub const QUERY_RESPONSE: &str = "queryResponse";

  use crate::messaging::{Message, MessageType, MessageContent, MessageError};
  use crate::session::SessionClient;
  use super::Plugin;
  
  pub struct TestPlugin {
    command_sent: bool,
    query_sent: bool,
  }

  impl TestPlugin {
    pub fn new() -> Self {
      Self {
	command_sent: false,
	query_sent: false,
      }
    }
  }

  impl Plugin for TestPlugin {
    fn accept_message(&mut self, mut message: Message) -> Result<(), MessageError> {
      match message.get_message_type() {
	MessageType::Query(id) => {
	  if QUERY_ID.as_bytes() == &id[0..QUERY_ID.len()] {
	    self.query_sent = true;
	    message.get_responder()
	      .expect("Should have responder")
	      .respond(MessageContent::from_content(QUERY_RESPONSE.to_string()))
	      .unwrap();
	  }
	},
	MessageType::Command(id) => {
	  if COMMAND_ID.as_bytes() == &id[0..COMMAND_ID.len()] {
	    self.command_sent = true;
	    message.get_responder()
	      .expect("Should have responder")
	      .respond(MessageContent::from_content(COMMAND_RESPONSE.to_string()))
	      .unwrap();
	  }
	},
      };

      Ok(())
    }

    fn list_message_types(&self) -> Vec<MessageType> {
      vec!(
	MessageType::command_from_str(COMMAND_ID).unwrap(),
	MessageType::query_from_str(QUERY_ID).unwrap(),
      )
    }

    fn receive_client(&mut self, _client: SessionClient) {
      ()
    }
  }
}
