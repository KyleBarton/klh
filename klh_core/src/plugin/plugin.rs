use crate::{event::{EventMessage, EventType}, session::SessionClient};


pub trait Plugin {

  fn accept_event(&mut self, event_message: EventMessage) -> Result<(), String>;

  fn list_event_types(&self) -> Vec<EventType>;

  fn receive_client(&mut self, client: SessionClient);

}
