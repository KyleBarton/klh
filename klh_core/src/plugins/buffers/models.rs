
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateBufferContent {
  pub name: String,
}


#[derive(Serialize, Deserialize)]
pub struct ListBuffersResponse {
  // TODO obviously
  pub list_as_string: String,
}
