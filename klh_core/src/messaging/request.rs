use bson::{Bson, Deserializer};
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot::{Sender, Receiver, self,};

use super::{Message, MessageType};

#[derive(Debug)]
pub struct MessageContent {
  content: Option<Bson>,
}

impl MessageContent {
  pub fn empty() -> Self {
    Self {
      content: None,
    }
  }
  pub fn from_content<T: Serialize>(content: T) -> Self {
    Self {
      content: Some(bson::to_bson(&content).unwrap()),
    }
  }

  pub fn deserialize<'a, T: Deserialize<'a>>(&mut self) -> Option<T> {
    match self.content.take() {
      None => None,
      Some(c) => {
	let content: T = {
	  let de = Deserializer::new(c);
	  Deserialize::deserialize(de)
	}.unwrap();

	Some(content)
      }
    }
  }
}

pub struct Request {
  message_type: MessageType,
  sender: Option<QueryResponder>,
  receiver: Option<QueryHandler>,
  content: Option<MessageContent>,
}

impl Request {
  pub fn new(message_type: MessageType, content: MessageContent) -> Self {
    let (tx, rx) = oneshot::channel();
    Self {
      message_type,
      sender: Some(QueryResponder::new(Some(tx))),
      receiver: Some(QueryHandler::new(Some(rx))),
      content: Some(content),
    }
  }

  pub fn from_id(id: &str) -> Self {
    let (tx, rx) = oneshot::channel();
    Self {
      message_type: MessageType::query_from_str(id),
      sender: Some(QueryResponder::new(Some(tx))),
      receiver: Some(QueryHandler::new(Some(rx))),
      content: None,
    }
  }

  pub fn to_message(&mut self) -> Result<Message, String> {
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

  pub fn get_handler(&mut self) -> Result<QueryHandler, String> {
    match self.receiver.take() {
      None => Err(String::from("Responder already taken")),
      Some(r) => Ok(r),
    }
    
  }
}

pub struct QueryHandler {
  receiver: Option<Receiver<QueryResponse>>,
}

impl QueryHandler {
  pub fn new(receiver: Option<Receiver<QueryResponse>>) -> Self {
    Self {
      receiver,
    }
  }
  pub async fn handle_response(&mut self) -> Result<QueryResponse, String> {
    match self.receiver.take() {
      None => Err("Already handled response".to_string()),
      Some(r) => {
	match r.await {
	  Ok(d) => Ok(d),
	  Err(err) => {
	    println!("{:?}", err);
	    Err("Problem receiving response".to_string())
	  },
	}
      }
    }
  }
}

#[derive(Debug)]
pub struct QueryResponder {
  sender: Option<Sender<QueryResponse>>,
}

impl QueryResponder {
  pub fn new(sender: Option<Sender<QueryResponse>>) -> Self {
    Self {
      sender,
    }
  }
  pub fn respond(&mut self, response: QueryResponse) -> Result<(), String> {
    match self.sender.take() {
      None => Err("Already responded".to_string()),
      Some(s) => {
	s.send(response).unwrap();
	Ok(())
      },
    }
  }
}

// TODO maybe just rename this to "Response"
#[derive(Debug)]
pub struct QueryResponse {
  content: String,
}

impl QueryResponse {
  pub fn from_str(content: &str) -> Self {
    Self {
      content: content.to_string(),
    }
  }

  pub fn as_string(&self) -> String {
    self.content.clone()
  }
}
