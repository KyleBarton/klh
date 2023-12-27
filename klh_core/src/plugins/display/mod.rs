use log::debug;

use crate::{plugin::Plugin, messaging::{MessageType, Message, MessageError}, session::SessionClient};

pub struct Display {
  message_types: Vec<MessageType>,
  // TODO move to klh client!
  session_client: Option<SessionClient>,
}

impl Display {
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
    }
  }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for Display {

    fn accept_message(&mut self, message: Message) -> Result<(), MessageError> {
      debug!("[DISPLAY] received message {}", message);
      match message.get_message_type() {
        // MessageType::Command("display::create_window") => {
        //   debug!("[DISPLAY] received create_window command");
        //   handle_create_window_command()
        // },
        // MessageType::Query("display::list_windows") => {
        //   debug!("[DISPLAY] received list_windows query");
        //   handle_list_windows_query()
        // },
        // MessageType::Command("display::delete_window") => {
        //   debug!("[DISPLAY] received delete_window command");
        //   handle_delete_window_command()
        // },
        // MessageType::Command("display::attach_buffer") => {
        //   debug!("[DISPLAY] received attach_buffer command");
        //   handle_attach_buffer_command()
        // },
        // MessageType::Command("display::detach_buffer") => {
        //   debug!("[DISPLAY] received detach_buffer command");
        //   handle_detach_buffer_command()
        // },
        // MessageType::Query("display::list_buffers_in_window") => {
        //   debug!("[DISPLAY] received list_buffers_in_window query");
        //   handle_list_buffers_in_window_query()
        // },
        _ => Ok(())
      }
    }

    fn list_message_types(&self) -> Vec<MessageType> {
        self.message_types.clone()
    }

    fn receive_client(&mut self, client: SessionClient) {
        self.session_client = Some(client);
    }
}
