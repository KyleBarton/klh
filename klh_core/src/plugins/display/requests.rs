
// requests.rs

use crate::messaging::{Request, MessageType};

// TODO visual positioning arguments?
// TODO window ID argument
pub fn new_create_window_request() -> Request {
  Request::from_message_type(
    MessageType::command_from_str("display::create_window").unwrap()
  )
}

// TODO window ID argument
pub fn new_delete_window_request() -> Request {
  Request::from_message_type(
    MessageType::command_from_str("display::delete_window").unwrap()
  )
}

// TODO window ID argument
// TODO buffer ID argument
pub fn new_attach_buffer_request() -> Request {
  Request::from_message_type(
    MessageType::command_from_str("display::attach_buffer").unwrap()
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
