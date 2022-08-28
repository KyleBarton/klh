use core::fmt;

use super::{MessageType, QueryResponder, MessageContent};

#[derive(Debug)]
pub struct Message {
  message_type: MessageType,
  responder: Option<QueryResponder>,
  content: Option<MessageContent>,
}

impl Message {

  pub fn new(
    message_type: MessageType,
    responder: Option<QueryResponder>,
    content: Option<MessageContent>,
  ) -> Self {
    Self {
      message_type,
      responder,
      content,
    }
  }

  pub fn get_message_type(&self) -> MessageType {
    self.message_type
  }

  pub fn get_responder(&mut self) -> Option<QueryResponder> {
    self.responder.take()
  }

  pub fn get_content(&mut self) -> Option<MessageContent> {
    self.content.take()
  }
}

impl fmt::Display for Message {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"Message {{
  message_type: {},
  content: {:?},
}}
", self.message_type.display_id(), self.content)
  }
}
