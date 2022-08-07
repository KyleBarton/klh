use core::fmt;
use std::hash::Hash;
use tokio::sync::oneshot::{Sender, Receiver, self,};


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EventType {
  Command([u8; 100]),
  Query([u8; 100]),
}

/// # Examples:
/// ```
/// use klh_core::event::EventType;
/// let event: EventType = EventType::query_from_str("test");
/// assert_eq!("test".to_string(), event.display_id())
/// ```
impl EventType {

  pub fn display_id(&self) -> String {
    match self {
      EventType::Command(bytes) => std::str::from_utf8(bytes)
	.unwrap()
	.replace("\u{0}", "")
	.to_string(),
      EventType::Query(bytes) => std::str::from_utf8(bytes)
	.unwrap()
	.replace("\u{0}", "")
	.to_string(),
    }
  }

  // TODO check for too long of a thing
  pub fn query_from_str(str_id: &str) -> Self {
    let mut id: [u8; 100] = [0u8; 100];
    let mut index = 0;
    for b in str_id.as_bytes() {
      id[index] = b.clone();
      index += 1;
    }

    Self::Query(id)
  }

  // TODO check for too long of a thing
  pub fn command_from_str(str_id: &str) -> Self {
    let mut id: [u8; 100] = [0u8; 100];
    let mut index = 0;
    for b in str_id.as_bytes() {
      id[index] = *b;
      index += 1;
    }

    Self::Command(id)
  }
}

impl fmt::Display for EventType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}", self.display_id())
  }
}



// TODO write a fmt::Display for this, it will be logged often
#[derive(Debug)]
pub struct EventMessage {
  event_type: EventType,
  responder: Option<QueryResponder>,
  // TODO, content needs much better structure than a string, but this
  // can get us started.
  content: Option<String>,
}

impl EventMessage {
  pub fn get_event_type(&self) -> EventType {
    self.event_type
  }

  pub fn get_responder(&mut self) -> Option<QueryResponder> {
    self.responder.take()
  }

  // TODO this is a use-once proposition; should it be?
  pub fn get_content(&mut self) -> Option<String> {
    self.content.take()
  }
}

impl fmt::Display for EventMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"EventMessage {{
  event_type: {},
  content: {:?}
}}
", self.event_type.display_id(), self.content)
  }
}

#[derive(Debug)]
pub struct QueryResponse {
  pub content: String,
}

pub struct BetterCommand {
  event_type: EventType,
  content: String,
}

impl BetterCommand {
  pub fn from_id(
    id: &str,
    content: String,
  ) -> Self {
    Self {
      event_type: EventType::command_from_str(id),
      content,
    }
  }

  pub fn get_event_message(&mut self) -> Result<EventMessage, String> {
    Ok(EventMessage {
      event_type: self.event_type,
      responder: None,
      content: Some(self.content.clone())
    })
  }
}

pub struct BetterQuery {
  event_type: EventType,
  sender: Option<QueryResponder>,
  receiver: Option<QueryHandler>,
}

impl BetterQuery {
  pub fn from_id(id: &str) -> Self {
    let (tx, rx) = oneshot::channel();
    Self {
      event_type: EventType::query_from_str(id),
      sender: Some(QueryResponder::new(Some(tx))),
      receiver: Some(QueryHandler::new(Some(rx))),
    }
  }

  pub fn get_event_message(&mut self) -> Result<EventMessage, String> {
    Ok(EventMessage {
      event_type: self.event_type,
      responder: self.sender.take(),
      // No content interface for queries, for now
      content: None,
    })
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
