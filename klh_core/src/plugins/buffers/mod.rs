use crate::{event::{EventType, EventMessage, QueryResponse, }, plugin::Plugin, session::SessionClient};

pub(crate) struct Buffers {
  event_types: Vec<EventType>,
  session_client: Option<SessionClient>,
  basic_buffer_names: Vec<String>,
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

  fn accept_event_v2(&mut self, mut event_message: EventMessage) -> Result<(), String> {
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

	  let response = QueryResponse {
	    content: content.clone()
	  };

	  event_message.get_responder().expect("No one should have used the event responder yet").respond(response).unwrap();
	}
	else {
	  println!("[BUFFERS] query not found");
	}
	Ok(())
      }
      EventType::Command(id) => {
	if &id[0.."buffers::create_buffer".len()] == "buffers::create_buffer".as_bytes() {
	  let buffer_name = event_message.get_content().expect("Should have a buffer name");
	  println!("[BUFFERS] Creating buffer with name {}", buffer_name);
	  self.basic_buffer_names.push(buffer_name);
	} else {
	  println!("[BUFFERS] command not found");
	}
	Ok(())
      }
    }
  }
}
