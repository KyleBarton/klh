use log::error;

use crate::messaging::Message;

use super::{dispatch::DispatchClient, SessionError};


/// A client by which KLH plugins can send
/// [Message](crate::messaging::Message)s to a running klh
/// [Session](super::Session). Instances of this struct should only be
/// provided by [Session::get_client](super::Session::get_client).
#[derive(Clone, Debug)]
pub struct SessionClient{
  dispatch_client: DispatchClient,
}

impl SessionClient {

  pub(crate) fn new(dispatch_client: DispatchClient) -> Self {
    Self {
      dispatch_client,
    }
  }

  ///Send a message to the running [Session](super::Session) which
  /// provided this instance of the client. This message will then be
  /// directed to the appropriate KLH plugin.
  /// # Errors
  ///Returns an [Err] result of [SessionError](super::SessionError) in
  /// certain situations:
  /// 1. [SessionError::ErrorSendingMessage](super::SessionError::ErrorSendingMessage)
  /// - the client was unable to send the message along to the running
  /// Session. Further details available in the error log.
  pub async fn send(&mut self, message: Message) -> Result<(), SessionError> {
    match self.dispatch_client.send(message).await {
      Err(err) => {
	error!("Error sending message to dispatch client: {}", err);
	Err(SessionError::ErrorSendingMessage)	
      },
      Ok(_) => Ok(())
    }
  }
}
