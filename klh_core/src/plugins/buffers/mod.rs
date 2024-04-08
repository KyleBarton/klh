use log::{debug, warn};

use crate::{messaging::{MessageType, Message, MessageContent, MessageError, }, plugin::Plugin, session::SessionClient};


pub mod requests;
pub mod models;

// TODO should this be a fully public struct? I think it should...
pub(crate) struct Buffers {
  message_types: Vec<MessageType>,
  session_client: Option<SessionClient>,
  basic_buffer_names: Vec<String>,
}

impl Buffers {
  pub fn new() -> Self {
    let message_types: Vec<MessageType> = vec![
      MessageType::command_from_str("buffers::create_buffer").unwrap(),
      MessageType::query_from_str("buffers::list_buffers").unwrap(),
    ];

    Self {
      message_types,
      session_client: None,
      basic_buffer_names: Vec::new(),
    }
  }
}

impl Default for Buffers {
  fn default() -> Self {
    Self::new()
  }
}

impl Plugin for Buffers {

  fn list_message_types(&self) -> Vec<MessageType> {
    self.message_types.clone()
  }

  fn receive_client(&mut self, session_client: SessionClient) {
    self.session_client = Some(session_client)
  }

  fn accept_message(&mut self, mut message: Message) -> Result<(), MessageError> {
    debug!("[BUFFERS] received message {}", message);
    let message_type = message.get_message_type();

    if message_type.id_equals_str("buffers::list_buffers") {
      let response = models::ListBuffersResponse {
	buffer_names: self.basic_buffer_names.clone(),
      };
      message.get_responder()
	.expect("No one should have used the responder yet")
	.respond(MessageContent::from_content(response))
	.unwrap();

    }

    else if message_type.id_equals_str("buffers::create_buffer") {
      let mut message_content = message.get_content().expect("Content should be present");
      let create_buffer_content : models::CreateBufferContent = message_content
	.deserialize()
	.expect("Should be able to deserialize");
      debug!("[BUFFERS] Creating buffer with name {}", &create_buffer_content.name);
      self.basic_buffer_names.push(create_buffer_content.name);
    }

    else {
      warn!("[BUFFERS] message type id not found: {}", &message.get_message_type());
    }
    Ok(())
  }
}
