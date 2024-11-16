use serde::{Serialize, Deserialize};

// TODO separate this into models.rs, requests.rs, and responses.rs
// Requests -> What subscribers care about
// Responses -> What clients care about
// Models -> The meat & potatoes, e.g. Buffer
// Eh deal with it when you have to. For now you might be able to avoid it.

#[derive(Serialize, Deserialize)]
pub struct CreateBufferRequest {
  pub name: String,
}

/// A simple way to append a string of characters to the end of the buffer. No cursor/point considered here.
#[derive(Serialize, Deserialize)]
pub struct AppendStringToBufferRequest {
  pub buffer_name: String,
  pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBufferRequest {
  pub buffer_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBufferResponse {
  pub buffer: Option<Buffer>,
}

#[derive(Serialize, Deserialize)]
pub struct ListBuffersResponse {
  pub buffer_names: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Buffer {
  pub name: String,
  pub content: BufferContent,
}

impl Buffer {
  pub fn new(name: String) -> Self {
    Self {
      name,
      content: BufferContent::new(),
    }
  }
}

// Placeholder for a way to store the content. For now basically wraps a string
#[derive(Serialize, Deserialize, Clone)]
pub struct BufferContent {
  content: String,
}

impl Default for BufferContent {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferContent {
  pub fn new() -> Self {
    Self {
      content: "".to_string(),
    }
  }

  pub fn append(&mut self, content: String) {
    self.content.push_str(&content);
  }
}
