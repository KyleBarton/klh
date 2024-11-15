use log::{debug, warn};
use models::{AttachBufferRequest, GetWindowRequest, GetWindowResponse};

use crate::{plugin::Plugin, messaging::{MessageType, Message, MessageError, MessageContent}, session::SessionClient, plugins::display::models::CreateWindowRequest};

use self::models::Window;

pub mod requests;
pub mod models;

pub struct Displays {
  message_types: Vec<MessageType>,
  // TODO move to klh client!
  session_client: Option<SessionClient>,
  windows: Vec<Window>,
}

impl Displays {
  pub fn new() -> Self {
    let message_types: Vec<MessageType> = vec![
      MessageType::command_from_str("display::create_window").unwrap(),
      MessageType::query_from_str("display::list_windows").unwrap(),
      MessageType::query_from_str("display::get_window").unwrap(),
      MessageType::command_from_str("display::delete_window").unwrap(),
      MessageType::command_from_str("display::attach_buffer").unwrap(),
      MessageType::command_from_str("display::detach_buffer").unwrap(),
      MessageType::query_from_str("display::list_buffers_in_window").unwrap(),
    ];

    Self {
      message_types,
      session_client: None,
      windows: Vec::new(),
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

	let response = models::ListWindowsResponse {
	  window_names: self.windows
	    .iter()
	    .map(|window| {
	      window.name.clone()
	    })
	    .collect()
	};
	message.get_responder()
	  .expect("No one should have used responder yet")
	  .respond(MessageContent::from_content(response))
	  .unwrap();
      }

      else if message_type.id_equals_str("display::create_window") {

	let mut request_content = message.get_content().expect("Got a window name");
	let request : CreateWindowRequest = request_content.deserialize().unwrap();

	if self.windows.iter().any(|window| window.name == request.window_name) {
	  return Err(MessageError::BadRequest(String::from("Window name already exists")))
	}

	self.windows.push(Window::new(request.window_name));
      }
      else if message_type.id_equals_str("display::attach_buffer") {
	let mut request_content = message.get_content().expect("Got a window name");
	let request : AttachBufferRequest = request_content.deserialize().unwrap();
	let window : &mut Window = self.windows.iter_mut().find(|window| window.name == request.window_name).expect("Found window");

	// TODO check with buffers plugin to ensure it's an actual buffer name
	window.associated_buffers_names.push(request.buffer_name.clone());
	// Assume this becomes the active buffer when this happens
	// Ok, what if I just re-order the vec? Idk figure this out later
	window.active_buffer_name = Some(request.buffer_name)
      }
      else if message_type.id_equals_str("display::get_window") {
	println!("hi");
	let mut request_content = message.get_content().expect("Got a window name");
	let request : GetWindowRequest = request_content.deserialize().unwrap();
	let window = self.windows.iter().find(|window| window.name == request.window_name);
	
	let response = GetWindowResponse {
	  window: window.cloned(),
	};
	message.get_responder()
	  .expect("No one should have used responder yet")
	  .respond(MessageContent::from_content(response))
	  .unwrap();
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
