use crate::messaging::Message;

use super::dispatch::DispatchClient;


pub struct SessionClient{
  dispatch_client: DispatchClient,
}

impl SessionClient {

  pub(crate) fn new(dispatch_client: DispatchClient) -> Self {
    Self {
      dispatch_client,
    }
  }

  pub async fn send(&mut self, message: Message) -> Result<(), String> {
    match self.dispatch_client.send(message).await {
      Err(_) => Err("Issue sending message to session".to_string()),
      Ok(_) => Ok(())
    }
  }
}
