use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ListWindowsResponse {
  pub list_as_string: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWindowRequest {
  pub window_name: String,
}
