use core::fmt;

use super::{EventType, QueryResponder, MessageContent};

#[derive(Debug)]
pub struct EventMessage {
  event_type: EventType,
  responder: Option<QueryResponder>,
  content: Option<MessageContent>,
}

impl EventMessage {

  pub fn new(
    event_type: EventType,
    responder: Option<QueryResponder>,
    content: Option<MessageContent>,
  ) -> Self {
    Self {
      event_type,
      responder,
      content,
    }
  }

  pub fn get_event_type(&self) -> EventType {
    self.event_type
  }

  pub fn get_responder(&mut self) -> Option<QueryResponder> {
    self.responder.take()
  }

  pub fn get_content(&mut self) -> Option<MessageContent> {
    self.content.take()
  }
}

impl fmt::Display for EventMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"EventMessage {{
  event_type: {},
  content: {:?},
}}
", self.event_type.display_id(), self.content)
  }
}
