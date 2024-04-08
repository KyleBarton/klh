use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ListWindowsResponse {
  pub window_names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWindowRequest {
  pub window_name: String,
}


pub struct Window {
  pub name: String,
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
