use log::{error, debug};
use tokio::sync::oneshot::{Sender, Receiver,};

use super::{MessageContent, MessageError};

/// A one-time use handler which awaits an asynchronous
/// [MessageContent] response to a [Request](super::Request) sent
/// through the klh session. No public constructor; Can only be
/// provided by [Request::get_handler](super::Request::get_handler)
#[derive(Debug)]
pub struct ResponseHandler {
  receiver: Option<Receiver<MessageContent>>,
}

impl ResponseHandler {
  pub(crate) fn new(receiver: Option<Receiver<MessageContent>>) -> Self {
    Self {
      receiver,
    }
  }

  /// Awaits a plugin response, returning a [MessageContent] result if
  /// successful. This method can only be called once int he lifetime
  /// of the response handler.
  /// # Errors
  /// [MessageError::ResponseAlreadyHandled] - The response handler
  /// has already been invoked
  /// [MessageError::FailedToReceiveResponse] - An error occurred
  /// while awaiting a response from the channel. The error will be
  /// logged at the [log::error] level.
  pub async fn handle_response(&mut self) -> Result<MessageContent, MessageError> {
    match self.receiver.take() {
      None => Err(MessageError::ResponseAlreadyHandled),
      Some(r) => {
	match r.await {
	  Ok(d) => Ok(d),
	  Err(err) => {
	    error!("Error receiving response: {:?}", err);
	    Err(MessageError::FailedToReceiveResponse)
	  },
	}
      }
    }
  }
}

/// A one-time use responder which a plugin can use in order to
/// respond to a [Request](super::Request) sent through a
/// [Message](super::Message). No constructor; can only be provided by
/// [Message::get_responder](super::Message::get_responder)
#[derive(Debug)]
pub struct Responder {
  sender: Option<Sender<MessageContent>>,
}

impl Responder {
  pub(crate) fn new(sender: Option<Sender<MessageContent>>) -> Self {
    Self {
      sender,
    }
    
  }

  /// One-time use function which responds on the request's oneshot
  /// callback channel with a [MessageContent] for the client to
  /// deserialize.
  /// # Errors
  /// `Err(MessagingError::RequestResponderAlreadyUsed` -> indicates
  /// that this function has already been called for this responder
  /// instance.
  pub fn respond(&mut self, response: MessageContent) -> Result<(), MessageError> {
    match self.sender.take() {
      None => {
	debug!("Attempted to repond with an already-used responder");
	Err(MessageError::ResponderAlreadyUsed)
      },
      Some(s) => {
	match s.send(response) {
	  Err(_) => Err(MessageError::FailedToSendResponse),
	  Ok(_) => Ok(()),
	}
      },
    }
  }
}

#[cfg(test)]
mod response_tests {
  use rstest::*;

use crate::messaging::{Request, MessageType, MessageContent, MessageError};

  #[tokio::test]
  async fn responder_should_respond_with_expected_content() {
    let mut given_request = Request::from_message_type(
      MessageType::query_from_str("test").unwrap()
    );

    let mut response_handler = given_request.get_handler().unwrap();

    let mut given_message = given_request.to_message();

    let mut responder = given_message.get_responder().expect("Should be a responder available");

    tokio::spawn(async move {
      responder.respond(MessageContent::from_content("responding"))
    });

    let response = response_handler.handle_response().await.unwrap();

    assert_eq!(
      response,
      MessageContent::from_content("responding"),
    )
  }

  #[rstest]
  fn responder_should_return_expected_error_when_called_to_respond_twice() {
    let mut given_request = Request::from_message_type(
      MessageType::query_from_str("test").unwrap()
    );
    let mut given_message = given_request.to_message();

    let mut responder = given_message.get_responder().expect("Should be a responder available");

    responder.respond(MessageContent::from_content("content")).unwrap();

    let second_response_attempt = responder.respond(MessageContent::from_content("content"));

    assert_eq!(
      second_response_attempt,
      Err(MessageError::ResponderAlreadyUsed),
    )
  }

  #[tokio::test]
  async fn response_hander_should_return_expected_error_when_called_to_handle_twice() {
    let mut given_request = Request::from_message_type(
      MessageType::query_from_str("test").unwrap()
    );

    let mut response_handler = given_request.get_handler().unwrap();

    let mut given_message = given_request.to_message();

    let mut responder = given_message.get_responder().expect("Should be a responder available");

    tokio::spawn(async move {
      responder.respond(MessageContent::from_content("responding"))
    });

    let first_response = response_handler.handle_response().await.unwrap();

    assert_eq!(
      first_response,
      MessageContent::from_content("responding"),
    );

    let second_response = response_handler.handle_response().await;

    assert_eq!(
      second_response,
      Err(MessageError::ResponseAlreadyHandled),
    )
  }
  
}
