use crate::{messaging::{Message, MessageType, MessageError}, session::SessionClient};


/// The trait that must be implemented to create a KLH Plugin.
pub trait Plugin {

  /// This is the single entrypoint by which the plugin is sent
  /// messages from the KLH runtime.
  fn accept_message(&mut self, message: Message) -> Result<(), MessageError>;

  /// This function must return all MessageType values that the Plugin
  /// wishes to register for. When the plugin is added to a KLH
  /// instance, the instance will call the plugin's list_message_types
  /// function in order to register message types that will be sent to
  /// this plugin.
  fn list_message_types(&self) -> Vec<MessageType>;

  /// This function is called by the KLH runtime to provide a copy of
  /// the [SessionClient](crate::session::SessionClient) with which the
  /// plugin can call the KLH API over the course of its functioning.
  fn receive_client(&mut self, client: SessionClient);

}
