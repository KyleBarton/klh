
use serde::{Serialize, Deserialize};

// TODO separate this into models.rs, requests.rs, and responses.rs
// Requests -> What subscribers care about
// Responses -> What clients care about
// Models -> The meat & potatoes, e.g. Buffer
// Eh deal with it when you have to. For now you might be able to avoid it.

#[derive(Serialize, Deserialize)]
pub struct CreateBufferContent {
  pub name: String,
}


#[derive(Serialize, Deserialize)]
pub struct ListBuffersResponse {
  pub buffer_names: Vec<String>,
}


pub struct Buffer {
  pub name: String
}

impl Buffer {
  pub fn new(name: String) -> Self {
    Self {
      name,
    }
  }
}
