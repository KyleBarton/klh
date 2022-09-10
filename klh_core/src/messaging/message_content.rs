use bson::{Bson, Deserializer};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone)]
pub struct MessageContent {
  content: Option<Bson>,
}

impl MessageContent {
  pub fn empty() -> Self {
    Self {
      content: None,
    }
  }
  pub fn from_content<T: Serialize>(content: T) -> Self {
    Self {
      content: Some(bson::to_bson(&content).unwrap()),
    }
  }

  pub fn deserialize<'a, T: Deserialize<'a>>(&mut self) -> Option<T> {
    match self.content.take() {
      None => None,
      Some(c) => {
	let de = Deserializer::new(c);
	match Deserialize::deserialize(de) {
	  // Return None if the type doesn't match
	  Err(_) => None,
	  Ok(content) => Some(content)
	}
      }
    }
  }
}
