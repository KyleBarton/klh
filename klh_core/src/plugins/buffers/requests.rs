use crate::messaging::{Request, MessageType, MessageContent};

use super::models::CreateBufferContent;

pub fn new_list_buffers_request() -> Request {
  Request::from_message_type(
    MessageType::query_from_str("buffers::list_buffers").unwrap()
  )
}

pub fn new_create_buffer_request(name: &str) -> Request {
  let create_buffer_content = CreateBufferContent {
    name: name.to_string(),
  };
  let content = MessageContent::from_content(create_buffer_content);

  Request::new(
    MessageType::command_from_str("buffers::create_buffer").unwrap(),
    content,
  )
}
