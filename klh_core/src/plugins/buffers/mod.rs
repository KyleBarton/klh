use log::{debug, warn};
use serde::{Serialize, Deserialize};

use crate::{messaging::{MessageType, Message, Request, MessageContent, }, plugin::Plugin, session::SessionClient};

pub(crate) struct Buffers {
  message_types: Vec<MessageType>,
  session_client: Option<SessionClient>,
  basic_buffer_names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBufferContent {
  name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListBuffersResponse {
  // TODO obviously
  pub list_as_string: String,
}

pub fn new_list_buffers_request() -> Request {
  Request::new(MessageType::query_from_str("buffers::list_buffers"), MessageContent::empty())
}

pub fn new_create_buffer_request(name: &str) -> Request {
  let create_buffer_content = CreateBufferContent {
    name: name.to_string(),
  };
  let content = MessageContent::from_content(create_buffer_content);

  Request::new(MessageType::command_from_str("buffers::create_buffer"), content)
}

impl Buffers {
  pub(crate) fn new() -> Self {
    let mut message_types: Vec<MessageType> = Vec::new();

    message_types.push(MessageType::command_from_str("buffers::create_buffer"));
    message_types.push(MessageType::query_from_str("buffers::list_buffers"));

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

  fn accept_message(&mut self, mut message: Message) -> Result<(), String> {
    debug!("[BUFFERS] received message");
    match message.get_message_type() {
      MessageType::Query(id) => {
	// TODO oh god this can be better. Really this is just enough
	// to prove that the async stuff is wired together. Much
	// refactoring needed here.
	if &id[0.."buffers::list_buffers".len()] == "buffers::list_buffers".as_bytes() {
	  debug!("[BUFFERS] got list buffers query via message!");

	  let mut content: String = "".to_string();

	  for buf_name in self.basic_buffer_names.iter() {
	    content.push_str(" ");
	    content.push_str(&buf_name);
	  }

	  // let response = ResponseMessage::from_str(&content);

	  // message.get_responder().expect("No one should have used the responder yet").respond(response).unwrap();
	  let response = ListBuffersResponse {
	    list_as_string: content,
	  };
	  message.get_responder()
	    .expect("No one should have used the responder yet")
	    .respond(MessageContent::from_content(response))
	    .unwrap();

	}
	else {
	  warn!("[BUFFERS] query not found");
	}
	Ok(())
      }
      MessageType::Command(id) => {
	if &id[0.."buffers::create_buffer".len()] == "buffers::create_buffer".as_bytes() {
	  let mut message_content = message.get_content().expect("Content should be present");
	  let create_buffer_content : CreateBufferContent = message_content
	    .deserialize()
	    .expect("Should be able to deserialize");
	  debug!("[BUFFERS] Creating buffer with name {}", &create_buffer_content.name);
	  self.basic_buffer_names.push(create_buffer_content.name);
	} else {
	  warn!("[BUFFERS] command not found");
	}
	Ok(())
      }
    }
  }
}
