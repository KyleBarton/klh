use core::fmt;

const MESSAGE_TYPE_ID_MAX_LENGTH: usize = 100;

/// A collection of error conditions that can occur when interacting with the [MessageType] struct.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageTypeError {
  /// Indicates an attempt to create a MessageType with an id that
  /// falls beyond the max length of 100.
  MessageTypeIdTooLong,
}

/// Enum that identifies the type of a message sent through Klh.  Note
/// that the difference between MessageType values is purely for
/// organization on the plugin side. A [Request](super::Request) will
/// be serialized into a [Message](super::Message) identically,
/// regardless ofthe value of its MessageType (e.g., the message will
/// always have a responder and the requests will always have a
/// handler).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MessageType {
  /// Intended for imperative requests to mutate state in a
  /// plugin. For instance, "buffers::create_buffer", from the Buffers
  /// plugin, is a command. Conventionally, clients should not expect
  /// [Message](super::Message) instances with a MessageType of
  /// Command to respond to the message.
  Command([u8; MESSAGE_TYPE_ID_MAX_LENGTH]),
  /// Intended to represent requests for data from a specific
  /// plugin. "buffers:list_buffers" is a query. Conventionally, it is
  /// expected that [Message](super::Message) instances with a
  /// MessageType of Query will respond via the
  /// [Responder](super::Responder) of a message.
  Query([u8; MESSAGE_TYPE_ID_MAX_LENGTH]),
}

/// # Examples:
/// ```
/// use klh_core::messaging::MessageType;
/// let message: MessageType = MessageType::query_from_str("test").unwrap();
/// assert_eq!("test".to_string(), message.display_id())
/// ```
impl MessageType {

  /// Displays the Id of the MessageType as a readable string, for
  /// debugging & logging purposes.
  pub fn display_id(&self) -> String {
    match self {
      MessageType::Command(bytes) => std::str::from_utf8(bytes)
	.unwrap()
	.replace("\u{0}", "")
	.to_string(),
      MessageType::Query(bytes) => std::str::from_utf8(bytes)
	.unwrap()
	.replace("\u{0}", "")
	.to_string(),
    }
  }

  /// A utility function to create a [MessageType::Query] from a &str
  /// input. The slice will be read as bytes and serialized into an ID
  /// of a fixed length in order to prevent dynamic sizing of
  /// MessageType objects.
  pub fn query_from_str(str_id: &str) -> Result<Self, MessageTypeError> {
    if str_id.len() > MESSAGE_TYPE_ID_MAX_LENGTH {
      Err(MessageTypeError::MessageTypeIdTooLong)
    }
    else {
      
      let mut id: [u8; MESSAGE_TYPE_ID_MAX_LENGTH] = [0u8; MESSAGE_TYPE_ID_MAX_LENGTH];
      let mut index = 0;
      for b in str_id.as_bytes() {
	id[index] = b.clone();
	index += 1;
      }

      Ok(Self::Query(id))
    }
  }


  /// A utility function to create a [MessageType::Command] from a
  /// &str input. The slice will be read as bytes and serialzied into
  /// an ID of a fixed lenght in order to prevent dynamic sizing of
  /// MessageType objects.
  pub fn command_from_str(str_id: &str) -> Result<Self, MessageTypeError> {
    if str_id.len() > MESSAGE_TYPE_ID_MAX_LENGTH {
      Err(MessageTypeError::MessageTypeIdTooLong)
    }
    else {
      let mut id: [u8; MESSAGE_TYPE_ID_MAX_LENGTH] = [0u8; MESSAGE_TYPE_ID_MAX_LENGTH];
      let mut index = 0;
      for b in str_id.as_bytes() {
	id[index] = *b;
	index += 1;
      }

      Ok(Self::Command(id))
    }
  }
}

impl fmt::Display for MessageType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}", self.display_id())
  }
}


#[cfg(test)]
mod message_type_tests {

  use rstest::*;

  use crate::messaging::MessageTypeError;

  use super::{MessageType, MESSAGE_TYPE_ID_MAX_LENGTH};


  #[rstest]
  fn should_create_command_from_str() {
    let message_type = MessageType::command_from_str("command_id").unwrap();

    assert!(matches!(message_type, MessageType::Command(..)));
    assert_eq!(message_type.display_id(), "command_id".to_string());
  }

  #[rstest]
  fn should_create_query_from_str() {
    let message_type = MessageType::query_from_str("query_id").unwrap();

    assert!(matches!(message_type, MessageType::Query(..)));
    assert_eq!(message_type.display_id(), "query_id".to_string());
  }

  #[rstest]
  fn should_fail_to_create_command_from_str_too_long() {
    let command_id_too_long: String = ['a'; MESSAGE_TYPE_ID_MAX_LENGTH+1].iter().collect();
    let message_type_result = MessageType::command_from_str(&command_id_too_long);
    assert!(message_type_result.is_err());
    assert_eq!(
      message_type_result.err().expect("Is an error"),
      MessageTypeError::MessageTypeIdTooLong,
    )
  }
  
  #[rstest]
  fn should_fail_to_create_query_from_str_too_long() {
    let query_id_too_long: String = ['a'; MESSAGE_TYPE_ID_MAX_LENGTH+1].iter().collect();
    let message_type_result = MessageType::query_from_str(&query_id_too_long);
    assert!(message_type_result.is_err());
    assert_eq!(
      message_type_result.err().expect("Is an error"),
      MessageTypeError::MessageTypeIdTooLong,
    )
  }
}
