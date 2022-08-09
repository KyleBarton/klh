use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EventType {
  Command([u8; 100]),
  Query([u8; 100]),
}

/// # Examples:
/// ```
/// use klh_core::event::EventType;
/// let event: EventType = EventType::query_from_str("test");
/// assert_eq!("test".to_string(), event.display_id())
/// ```
impl EventType {

  pub fn display_id(&self) -> String {
    match self {
      EventType::Command(bytes) => std::str::from_utf8(bytes)
	.unwrap()
	.replace("\u{0}", "")
	.to_string(),
      EventType::Query(bytes) => std::str::from_utf8(bytes)
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

impl fmt::Display for EventType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}", self.display_id())
  }
}



