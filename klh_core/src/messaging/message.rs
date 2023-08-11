use core::fmt;

use super::{MessageType, Responder, MessageContent};

/// The fundamental struct by which plugins accept data through the
/// [Plugin::accept_message](crate::plugin::Plugin::accept_message)
/// interface. The plugin contains the necessary information for a
/// plugin to get the [MessageType] of the originating request, the
/// [MessageContent] of the message, and a [Responder] with
/// which to send an asynchronous response.
#[derive(Debug)]
pub struct Message {
  message_type: MessageType,
  responder: Option<Responder>,
  content: Option<MessageContent>,
}

impl Message {

  pub(crate) fn new(
    message_type: MessageType,
    responder: Option<Responder>,
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
  pub fn get_responder(&mut self) -> Option<Responder> {
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

#[cfg(test)]
mod message_tests {
  use rstest::*;

  use crate::messaging::{MessageType, MessageContent, Request};

  #[rstest]
  fn should_get_expected_content_from_message() {
    let mut given_request = Request::new(
      MessageType::command_from_str("command").unwrap(),
      MessageContent::from_content("content"),
    );

    let mut message = given_request.as_message();

    assert_eq!(
      message.get_content(),
      Some(MessageContent::from_content("content")),
    )
  }

  #[rstest]
  fn should_preserve_message_content_after_providing() {
    let mut given_request = Request::new(
      MessageType::command_from_str("command").unwrap(),
      MessageContent::from_content("content"),
    );

    let mut message = given_request.as_message();

    let _content_throwaway = message.get_content();

    let preserved_content = message.get_content();

    assert_eq!(
      preserved_content,
      Some(MessageContent::from_content("content")),
    )
  }

  #[rstest]
  fn should_provide_responder() {
    let mut given_request = Request::new(
      MessageType::command_from_str("command").unwrap(),
      MessageContent::from_content("content"),
    );

    let mut message = given_request.as_message();

    let responder = message.get_responder();

    assert!(responder.is_some())
  }

  #[rstest]
  fn should_provide_none_if_asked_for_responder_twice() {
    let mut given_request = Request::new(
      MessageType::command_from_str("command").unwrap(),
      MessageContent::from_content("content"),
    );

    let mut message = given_request.as_message();

    let _responder_thrown_away = message.get_responder();

    let second_responder = message.get_responder();

    assert!(second_responder.is_none())
  }

  #[rstest]
  fn should_return_expected_message_type() {
    let mut given_request = Request::from_message_type(
      MessageType::query_from_str("query").unwrap(),
    );

    let message = given_request.as_message();

    assert_eq!(
      message.get_message_type(),
      MessageType::query_from_str("query").unwrap(),
    )
  }
}
