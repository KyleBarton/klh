use log::error;

use crate::messaging::Message;

use super::{dispatch::DispatchClient, SessionError};


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
