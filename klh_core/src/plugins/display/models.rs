use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ListWindowsResponse {
  pub window_names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWindowRequest {
  pub window_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AttachBufferRequest {
  pub window_name: String,
  pub buffer_name: String,
}


#[derive(Debug)]
pub struct Window {
  pub name: String,
  // TODO figure out how to use lifetimes to tie the buffer name to the vec
  pub active_buffer_name: Option<String>,
  pub associated_buffers_names: Vec<String>,
}

impl Window {
  pub fn new(name: String) -> Self {
    Self {
      name,
      active_buffer_name: None,
      associated_buffers_names: Vec::new(),
    }
  }
}
