
// requests.rs

use crate::messaging::{Request, MessageType, MessageContent};

use super::models::{AttachBufferRequest, CreateWindowRequest};

// TODO visual positioning arguments?
// TODO window ID argument
pub fn new_create_window_request(win_name: &str) -> Request {
  let create_window_content = CreateWindowRequest {
    window_name: win_name.to_string(),
  };
  Request::new(
    MessageType::command_from_str("display::create_window").unwrap(),
    MessageContent::from_content(create_window_content),
  )
}

// TODO window ID argument
pub fn new_delete_window_request() -> Request {
  Request::from_message_type(
    MessageType::command_from_str("display::delete_window").unwrap()
  )
}

pub fn new_attach_buffer_request(
  buffer_name: &str,
  window_name: &str,
) -> Request {
  let attach_buffer_request = AttachBufferRequest {
    window_name: window_name.to_string(),
    buffer_name: buffer_name.to_string(),
  };
  Request::new(
    MessageType::command_from_str("display::attach_buffer").unwrap(),
    MessageContent::from_content(attach_buffer_request),
  )
}

// TODO window ID argument
// TODO buffer ID argument
pub fn new_detach_buffer_request() -> Request {
  Request::from_message_type(
    MessageType::command_from_str("display::detach_buffer").unwrap()
  )
}

// TODO window ID argument
pub fn new_list_buffers_in_window() -> Request {
  Request::from_message_type(
    MessageType::query_from_str("display::list_buffers_in_window").unwrap()
  )
}

pub fn new_list_windows_request() -> Request {
  Request::from_message_type(
    MessageType::query_from_str("display::list_windows").unwrap()
  )
}
