use crate::messaging::{Request, MessageType, MessageContent};

use super::models::{AppendStringToBufferRequest, CreateBufferRequest, GetBufferRequest};

pub fn new_list_buffers_request() -> Request {
  Request::from_message_type(
    MessageType::command_from_str("buffers::list_buffers").unwrap()
  )
}

pub fn new_create_buffer_request(name: &str) -> Request {
  let create_buffer_content = CreateBufferRequest {
    name: name.to_string(),
  };
  let content = MessageContent::from_content(create_buffer_content);

  Request::new(
    MessageType::command_from_str("buffers::create_buffer").unwrap(),
    content,
  )
}

pub fn new_append_string_to_buffer_request(buffer_name: &str, content: &str) -> Request {
  let append_request = AppendStringToBufferRequest {
    buffer_name: buffer_name.to_string(),
    content: content.to_string(),
  };
  Request::new(
    MessageType::command_from_str("buffers:append_string").unwrap(),
    MessageContent::from_content(append_request),
  )
}

pub fn new_get_buffer_request(name: &str) -> Request {
  let get_buffer_request = GetBufferRequest {
    buffer_name: name.to_string(),
  };

  Request::new(
    MessageType::command_from_str("buffers::get_buffer").unwrap(),
    MessageContent::from_content(get_buffer_request),
  )
}
