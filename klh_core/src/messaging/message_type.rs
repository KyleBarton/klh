use core::fmt;

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
  Command([u8; 100]),
  /// Intended to represent requests for data from a specific
  /// plugin. "buffers:list_buffers" is a query. Conventionally, it is
  /// expected that [Message](super::Message) instances with a
  /// MessageType of Query will respond via the
  /// [Responder](super::Responder) of a message.
  Query([u8; 100]),
}

/// # Examples:
/// ```
/// use klh_core::messaging::MessageType;
/// let message: MessageType = MessageType::query_from_str("test");
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
  pub fn query_from_str(str_id: &str) -> Self {
    let mut id: [u8; 100] = [0u8; 100];
    let mut index = 0;
    for b in str_id.as_bytes() {
      id[index] = b.clone();
      index += 1;
    }

    Self::Query(id)
  }

  /// A utility function to create a [MessageType::Command] from a
  /// &str input. The slice will be read as bytes and serialzied into
  /// an ID of a fixed lenght in order to prevent dynamic sizing of
  /// MessageType objects.
  pub fn command_from_str(str_id: &str) -> Self {
    let mut id: [u8; 100] = [0u8; 100];
    let mut index = 0;
    for b in str_id.as_bytes() {
      id[index] = *b;
      index += 1;
    }

    Self::Command(id)
  }
}

impl fmt::Display for MessageType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}", self.display_id())
  }
}



