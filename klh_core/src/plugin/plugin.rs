use crate::{messaging::{Message, MessageType}, session::SessionClient};


pub trait Plugin {

  fn accept_message(&mut self, message: Message) -> Result<(), String>;

  fn list_message_types(&self) -> Vec<MessageType>;

  fn receive_client(&mut self, client: SessionClient);

}
