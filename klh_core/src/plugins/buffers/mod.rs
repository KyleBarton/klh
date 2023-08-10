use log::{debug, warn};

use crate::{messaging::{MessageType, Message, MessageContent, MessageError, }, plugin::Plugin, session::SessionClient};


pub mod requests;
pub mod models;

pub(crate) struct Buffers {
  message_types: Vec<MessageType>,
  session_client: Option<SessionClient>,
  basic_buffer_names: Vec<String>,
}

impl Buffers {
  pub fn new() -> Self {
    let mut message_types: Vec<MessageType> = Vec::new();

    message_types.push(MessageType::command_from_str("buffers::create_buffer").unwrap());
    message_types.push(MessageType::query_from_str("buffers::list_buffers").unwrap());

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
    match message.get_message_type() {
      MessageType::Query(id) => {
	// TODO oh god this can be better. Really this is just enough
	// to prove that the async stuff is wired together. Much
	// refactoring needed here.
	if &id[0.."buffers::list_buffers".len()] == "buffers::list_buffers".as_bytes() {
	  let mut content: String = "".to_string();

	  for buf_name in self.basic_buffer_names.iter() {
	    content.push_str(" ");
	    content.push_str(&buf_name);
	  }

	  let response = models::ListBuffersResponse {
	    list_as_string: content,
	  };
	  message.get_responder()
	    .expect("No one should have used the responder yet")
	    .respond(MessageContent::from_content(response))
	    .unwrap();

	}
	else {
	  warn!("[BUFFERS] message type id not found: {}", &message.get_message_type());
	}
	Ok(())
      }
      MessageType::Command(id) => {
	if &id[0.."buffers::create_buffer".len()] == "buffers::create_buffer".as_bytes() {
	  let mut message_content = message.get_content().expect("Content should be present");
	  let create_buffer_content : models::CreateBufferContent = message_content
	    .deserialize()
	    .expect("Should be able to deserialize");
	  debug!("[BUFFERS] Creating buffer with name {}", &create_buffer_content.name);
	  self.basic_buffer_names.push(create_buffer_content.name);
	} else {
	  warn!("[BUFFERS] message type id not found: {}", &message.get_message_type());
	}
	Ok(())
      }
    }
  }
}
