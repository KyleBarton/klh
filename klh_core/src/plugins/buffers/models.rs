
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateBufferContent {
  pub name: String,
}


#[derive(Serialize, Deserialize)]
pub struct ListBuffersResponse {
  pub buffer_names: Vec<String>,
}
