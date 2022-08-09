// query.rs
use tokio::sync::oneshot::{Sender, Receiver, self,};

use super::{EventMessage, EventType};


pub struct Query {
  event_type: EventType,
  sender: Option<QueryResponder>,
  receiver: Option<QueryHandler>,
}

impl Query {
  pub fn from_id(id: &str) -> Self {
    let (tx, rx) = oneshot::channel();
    Self {
      event_type: EventType::query_from_str(id),
      sender: Some(QueryResponder::new(Some(tx))),
      receiver: Some(QueryHandler::new(Some(rx))),
    }
  }

  pub fn get_event_message(&mut self) -> Result<EventMessage, String> {
    Ok(EventMessage::new(
      self.event_type,
      self.sender.take(),
      // No content interface for queries, for now
      None,
    ))
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

#[derive(Debug)]
pub struct QueryResponse {
  pub content: String,
}
