use bson::{Bson, Deserializer};
use serde::{Serialize, Deserialize};


/// A serialized representation of structured content passed through
/// Request & Response messages. This struct relies on the
/// [Serialize](serde::Serialize) and
/// [Deserialize](serde::Deserialize) traits for provided generic
/// types.
#[derive(Debug, Clone)]
pub struct MessageContent {
  content: Option<Bson>,
}

impl MessageContent {
  /// Creates an empty [MessageContent] struct instance.
  pub fn empty() -> Self {
    Self {
      content: None,
    }
  }

  /// Creates a [MessageContent] instance which serializes the given type.
  /// # Examples
  /// ```
  /// use klh_core::messaging::MessageContent;
  /// let original_content = "This is some content".to_string();
  /// let msg_content: MessageContent = MessageContent::from_content(original_content);
  /// ```
  pub fn from_content<T: Serialize>(content: T) -> Self {
    Self {
      content: Some(bson::to_bson(&content).unwrap()),
    }
  }

  /// One-time use function which returns an instance of type `T` if
  /// successful.  Returns [None] if serialization
  /// is not possible for the following reasons:
  /// - The content was already deserialized
  /// - The serialized content does not fit the provided type to
  /// deserialize.
  /// # Examples
  /// ```
  /// use klh_core::messaging::MessageContent;
  /// let mut msg_content = MessageContent::from_content("This is content".to_string());
  /// let content: String = msg_content.deserialize().expect("Should be a string content");
  /// assert_eq!(content, "This is content".to_string());
  /// ```
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
