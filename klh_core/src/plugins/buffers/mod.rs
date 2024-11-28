use log::{debug, error, warn};
use models::GetBufferResponse;

use crate::{messaging::{CommandMessage, Event, Message, MessageContent, MessageError, MessageType}, plugin::Plugin, session::SessionClient};

use self::models::Buffer;


pub mod requests;
pub mod models;

// TODO should this be a fully public struct? I think it should...
pub(crate) struct Buffers {
  message_types: Vec<MessageType>,
  session_client: Option<SessionClient>,
  buffers: Vec<Buffer>,
}

impl Buffers {
  pub fn new() -> Self {
    let message_types: Vec<MessageType> = vec![
      MessageType::command_from_str("buffers::create_buffer").unwrap(),
      MessageType::query_from_str("buffers::list_buffers").unwrap(),
      MessageType::query_from_str("buffers::get_buffer").unwrap(),
      MessageType::command_from_str("buffers:append_string").unwrap(),
    ];

    Self {
      message_types,
      session_client: None,
      buffers: Vec::new(),
    }
  }
  fn accept_command(&mut self, mut message: CommandMessage) -> Result<(), MessageError> {
    debug!("[BUFFERS] received message {:?}", message);
    let message_type = message.get_message_type();

    if message_type.id_equals_str("buffers::list_buffers") {
      let response = models::ListBuffersResponse {
	buffer_names: self.buffers.iter().map(|b| b.name.clone()).collect(),
      };
      if let Err(e) = message.get_responder()
	.expect("No one should have used the responder yet")
	.respond(MessageContent::from_content(response)) {
	  error!("[BUFFERS] Unable to respond to message. Error: {:?}", e);
	  return Err(MessageError::PluginFailedToProcessMessage);
	}

    }

    else if message_type.id_equals_str("buffers::create_buffer") {
      let mut message_content = message.get_content().expect("Content should be present");
      let create_buffer_content : models::CreateBufferRequest = message_content
	.deserialize()
	.expect("Should be able to deserialize");

      debug!("[BUFFERS] Creating buffer with name {}", &create_buffer_content.name);

      if self.buffers.iter().any(|b| b.name == create_buffer_content.name) {
	return Err(MessageError::BadRequest(String::from("Buffer name already exists")))
      }

      self.buffers.push(Buffer::new(create_buffer_content.name));
    }

    else if message_type.id_equals_str("buffers:append_string") {
      let mut message_content = message.get_content().expect("Content should be present");
      let append_buffer_content : models::AppendStringToBufferRequest = message_content
	.deserialize()
	.expect("Should be able to deserialize");

      let buffer = match self.buffers.iter_mut().find(|b| b.name == append_buffer_content.buffer_name) {
        Some(buffer) => buffer,
        None => return Err(MessageError::BadRequest(String::from("No buffer matching provided name"))),
      };

      buffer.content.append(append_buffer_content.content);

      //TODO this is meh
      let mut request = Event::new(
	MessageType::event_from_str("buffers:buffer_appended").unwrap(),
	MessageContent::from_content(buffer),
      );

      let client_clone = self.session_client.clone().expect("Should have a client");

      tokio::spawn(async move {
	client_clone.send(request.as_message()).await.unwrap();
      });
    }

    else if message_type.id_equals_str("buffers::get_buffer") {
      let mut message_content = message.get_content().expect("Content should be present");
      let get_buffer_request : models::GetBufferRequest = message_content
	.deserialize()
	.expect("Should be able to deserialize");
      
      let buffer = self.buffers.iter().find(|b| b.name == get_buffer_request.buffer_name);
      if let Err(e) = message.get_responder()
	.expect("No one should have used the responder yet")
	.respond(MessageContent::from_content(GetBufferResponse { buffer: buffer.cloned() })) {
	  error!("[BUFFERS] Unable to respond to message. Error: {:?}", e);
	  return Err(MessageError::PluginFailedToProcessMessage);
	}
    }

    else {
      warn!("[BUFFERS] message type id not found: {}", &message.get_message_type());
    }
    Ok(())
  }
}

impl Default for Buffers {
  fn default() -> Self {
    Self::new()
  }
}

impl Plugin for Buffers {

  fn list_message_types(&self) -> Vec<MessageType> {
    self.message_types.clone()
  }

  fn receive_client(&mut self, session_client: SessionClient) {
    self.session_client = Some(session_client)
  }

  fn accept_message(&mut self, message: Message) -> Result<(), MessageError> {
    match message {
        Message::Event(_) => todo!(),
        Message::Command(command) => self.accept_command(command),
    }
  }
}
