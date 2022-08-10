use crate::event::EventMessage;

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

  pub async fn send(&mut self, event_message: EventMessage) -> Result<(), String> {
    match self.dispatch_client.send(event_message).await {
      Err(_) => Err("Issue sending event message to session".to_string()),
      Ok(_) => Ok(())
    }
  }
}
