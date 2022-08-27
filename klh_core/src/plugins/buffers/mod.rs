use serde::{Serialize, Deserialize};

use crate::{event::{EventType, EventMessage, QueryResponse, Request, MessageContent, }, plugin::Plugin, session::SessionClient};

pub(crate) struct Buffers {
  event_types: Vec<EventType>,
  session_client: Option<SessionClient>,
  basic_buffer_names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBufferContent {
  name: String,
}

pub fn new_list_buffers_request() -> Request {
  Request::new(EventType::query_from_str("buffers::list_buffers"), MessageContent::empty())
}

pub fn new_create_buffer_request(name: &str) -> Request {
  // Request::new(EventType::command_from_str("buffers::create_buffer"), RequestContent::from_string(name.to_string()))
  let create_buffer_content = CreateBufferContent {
    name: name.to_string(),
  };
  let content = MessageContent::from_content(create_buffer_content);

  Request::new(EventType::command_from_str("buffers::create_buffer"), content)
}

impl Buffers {
  pub(crate) fn new() -> Self {
    let mut event_types: Vec<EventType> = Vec::new();

    event_types.push(EventType::command_from_str("buffers::create_buffer"));
    event_types.push(EventType::query_from_str("buffers::list_buffers"));

    Self {
      event_types,
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

  fn list_event_types(&self) -> Vec<EventType> {
    self.event_types.clone()
  }

  fn receive_client(&mut self, session_client: SessionClient) {
    self.session_client = Some(session_client)
  }

  fn accept_event(&mut self, mut event_message: EventMessage) -> Result<(), String> {
    println!("[BUFFERS] received event message");
    match event_message.get_event_type() {
      EventType::Query(id) => {
	// TODO oh god this can be better. Really this is just enough
	// to prove that the async stuff is wired together. Much
	// refactoring needed here.
	if &id[0.."buffers::list_buffers".len()] == "buffers::list_buffers".as_bytes() {
	  println!("[BUFFERS] got list buffers query via event message!");

	  let mut content: String = "".to_string();

	  for buf_name in self.basic_buffer_names.iter() {
	    content.push_str(" ");
	    content.push_str(&buf_name);
	  }

	  let response = QueryResponse::from_str(&content);

	  event_message.get_responder().expect("No one should have used the event responder yet").respond(response).unwrap();
	}
	else {
	  println!("[BUFFERS] query not found");
	}
	Ok(())
      }
      EventType::Command(id) => {
	if &id[0.."buffers::create_buffer".len()] == "buffers::create_buffer".as_bytes() {
	  // let buffer_name = event_message.get_content().expect("Should have a buffer name");
	  let mut message_content = event_message.get_content().expect("Content should be present");
	  let create_buffer_content : CreateBufferContent = message_content.deserialize().expect("Should be able to deserialize");
	  println!("[BUFFERS] Creating buffer with name {}", &create_buffer_content.name);
	  self.basic_buffer_names.push(create_buffer_content.name);
	} else {
	  println!("[BUFFERS] command not found");
	}
	Ok(())
      }
    }
  }
}
