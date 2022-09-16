use core::fmt;

use super::{MessageType, RequestResponder, MessageContent};

/// The fundamental struct by which plugins accept data through the
/// [Plugin::accept_message](crate::plugin::Plugin::accept_message)
/// interface. The plugin contains the necessary information for a
/// plugin to get the [MessageType] of the originating request, the
/// [MessageContent] of the message, and a [RequestResponder] with
/// which to send an asynchronous response.
#[derive(Debug)]
pub struct Message {
  message_type: MessageType,
  responder: Option<RequestResponder>,
  content: Option<MessageContent>,
}

impl Message {

  pub(crate) fn new(
    message_type: MessageType,
    responder: Option<RequestResponder>,
    content: Option<MessageContent>,
  ) -> Self {
    Self {
      message_type,
      responder,
      content,
    }
  }

  /// Returns the [MessageType] associated with the Message.
  pub fn get_message_type(&self) -> MessageType {
    self.message_type
  }

  /// Gets a one-time use responder with which to asynchronously
  /// respond to the message. A `Message` instance returns [None] if
  /// this function is called more than once.
  pub fn get_responder(&mut self) -> Option<RequestResponder> {
    self.responder.take()
  }

  /// Gets the [MessageContent] of the message, if present. Otherwise,
  /// returns [None].
  pub fn get_content(&mut self) -> Option<MessageContent> {
    match self.content.take() {
      None => None,
      Some(c) => {
	self.content = Some(c.clone());
	Some(c)
      }
    }
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
