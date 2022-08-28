use bson::{Bson, Deserializer};
use serde::{Serialize, Deserialize};


#[derive(Debug)]
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
	let content: T = {
	  let de = Deserializer::new(c);
	  Deserialize::deserialize(de)
	}.unwrap();

	Some(content)
      }
    }
  }
}
