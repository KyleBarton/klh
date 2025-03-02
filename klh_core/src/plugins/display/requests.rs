use crate::messaging::{MessageContent, MessageType, Request};

use super::models::{
    AcceptStringInputRequest, AttachBufferRequest, CreateWindowRequest, GetWindowRequest,
};

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
    Request::from_message_type(MessageType::command_from_str("display::delete_window").unwrap())
}

pub fn new_attach_buffer_request(buffer_name: &str, window_name: &str) -> Request {
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
    Request::from_message_type(MessageType::command_from_str("display::detach_buffer").unwrap())
}

// TODO window ID argument
pub fn new_list_buffers_in_window() -> Request {
    Request::from_message_type(
        MessageType::command_from_str("display::list_buffers_in_window").unwrap(),
    )
}

pub fn new_list_windows_request() -> Request {
    Request::from_message_type(MessageType::command_from_str("display::list_windows").unwrap())
}

pub fn new_get_window_request(window_name: &str) -> Request {
    let get_window_request = GetWindowRequest {
        window_name: window_name.to_string(),
    };
    Request::new(
        MessageType::command_from_str("display::get_window").unwrap(),
        MessageContent::from_content(get_window_request),
    )
}

pub fn new_accept_string_input_request(window_name: &str, input: &str) -> Request {
    let accept_input_request = AcceptStringInputRequest {
        window_name: window_name.to_string(),
        input: input.to_string(),
    };
    Request::new(
        MessageType::command_from_str("display::accept_string_input").unwrap(),
        MessageContent::from_content(accept_input_request),
    )
}
