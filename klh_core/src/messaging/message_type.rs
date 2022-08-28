use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MessageType {
  Command([u8; 100]),
  Query([u8; 100]),
}

/// # Examples:
/// ```
/// use klh_core::messaging::MessageType;
/// let message: MessageType = MessageType::query_from_str("test");
/// assert_eq!("test".to_string(), message.display_id())
/// ```
impl MessageType {

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

  // TODO check for too long of a thing
  pub fn query_from_str(str_id: &str) -> Self {
    let mut id: [u8; 100] = [0u8; 100];
    let mut index = 0;
    for b in str_id.as_bytes() {
      id[index] = b.clone();
      index += 1;
    }

    Self::Query(id)
  }

  // TODO check for too long of a thing
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



