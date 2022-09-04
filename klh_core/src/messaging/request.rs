use tokio::sync::oneshot;

use super::{Message, MessageType, MessageContent, RequestResponder, ResponseHandler};

pub struct Request {
  message_type: MessageType,
  sender: Option<RequestResponder>,
  receiver: Option<ResponseHandler>,
  content: Option<MessageContent>,
}

impl Request {
  pub fn new(message_type: MessageType, content: MessageContent) -> Self {
    let (tx, rx) = oneshot::channel();
    Self {
      message_type,
      sender: Some(RequestResponder::new(Some(tx))),
      receiver: Some(ResponseHandler::new(Some(rx))),
      content: Some(content),
    }
  }

  pub fn from_message_type(message_type: MessageType) -> Self {
    let (tx, rx) = oneshot::channel();
    Self {
      message_type,
      sender: Some(RequestResponder::new(Some(tx))),
      receiver: Some(ResponseHandler::new(Some(rx))),
      content: None,
    }
  }

  pub(crate) fn to_message(&mut self) -> Result<Message, String> {
    match self.content.take() {
      None => Ok(Message::new(
	self.message_type,
	self.sender.take(),
	None,
      )),
      Some(content) => Ok(Message::new(
	self.message_type,
	self.sender.take(),
	Some(content),
      ))
    }
    
  }

  pub fn get_handler(&mut self) -> Result<ResponseHandler, String> {
    match self.receiver.take() {
      None => Err(String::from("Responder already taken")),
      Some(r) => Ok(r),
    }
    
  }
}

