use log::{debug, warn};

use crate::{plugin::Plugin, messaging::{MessageType, Message, MessageError, MessageContent}, session::SessionClient, plugins::display::models::CreateWindowRequest};

pub mod requests;
pub mod models;

pub struct Displays {
  message_types: Vec<MessageType>,
  // TODO move to klh client!
  session_client: Option<SessionClient>,
  basic_window_names: Vec<String>,
}

impl Displays {
  pub fn new() -> Self {
    let message_types: Vec<MessageType> = vec![
      MessageType::command_from_str("display::create_window").unwrap(),
      MessageType::query_from_str("display::list_windows").unwrap(),
      MessageType::command_from_str("display::delete_window").unwrap(),
      MessageType::command_from_str("display::attach_buffer").unwrap(),
      MessageType::command_from_str("display::detach_buffer").unwrap(),
      MessageType::query_from_str("display::list_buffers_in_window").unwrap(),
    ];

    Self {
      message_types,
      session_client: None,
      basic_window_names: Vec::new(),
    }
  }
}

impl Default for Displays {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for Displays {

    fn accept_message(&mut self, mut message: Message) -> Result<(), MessageError> {
      debug!("[DISPLAY] received message {}", message);
      let message_type = message.get_message_type();

      if message_type.id_equals_str("display::list_windows") {
	let mut content: String = "".to_string();

	for win_name in self.basic_window_names.iter() {
	  content.push(' ');
	  content.push_str(win_name);
	}

	let response = models::ListWindowsResponse {
	  list_as_string: content
	};
	message.get_responder()
	  .expect("No one should have used responder yet")
	  .respond(MessageContent::from_content(response))
	  .unwrap();
      }

      else if message_type.id_equals_str("display::create_window") {
	let mut request_content = message.get_content().expect("Got a window name");
	let request : CreateWindowRequest = request_content.deserialize().unwrap();
	self.basic_window_names.push(request.window_name);
      }

      else {
	warn!("[DISPLAY] message type id not found: {}", &message.get_message_type());
      }

      Ok(())
    }

    fn list_message_types(&self) -> Vec<MessageType> {
        self.message_types.clone()
    }

    fn receive_client(&mut self, client: SessionClient) {
        self.session_client = Some(client);
    }
}
