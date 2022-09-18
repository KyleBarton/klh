use log::error;
use tokio::sync::oneshot::{Sender, Receiver,};

use super::{MessageContent, MessageError};

/// A one-time use handler which awaits an asynchronous
/// [MessageContent] response to a [Request](super::Request) sent
/// through the klh session. No public constructor; Can only be
/// provided by [Request::get_handler](super::Request::get_handler)
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
pub struct RequestResponder {
  sender: Option<Sender<MessageContent>>,
}

impl RequestResponder {
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
      None => Err(MessageError::RequestResponderAlreadyUsed),
      Some(s) => {
	s.send(response).unwrap();
	Ok(())
      },
    }
  }
}
