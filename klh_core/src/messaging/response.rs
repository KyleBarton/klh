use log::error;
use tokio::sync::oneshot::{Sender, Receiver,};

use super::MessageContent;

pub struct ResponseHandler {
  receiver: Option<Receiver<MessageContent>>,
}

impl ResponseHandler {
  pub(crate) fn new(receiver: Option<Receiver<MessageContent>>) -> Self {
    Self {
      receiver,
    }
  }
  pub async fn handle_response(&mut self) -> Result<MessageContent, String> {
    match self.receiver.take() {
      None => Err("Already handled response".to_string()),
      Some(r) => {
	match r.await {
	  Ok(d) => Ok(d),
	  Err(err) => {
	    error!("Error receiving response: {:?}", err);
	    Err("Problem receiving response".to_string())
	  },
	}
      }
    }
  }
}

#[derive(Debug)]
pub struct RequestResponder {
  sender: Option<Sender<MessageContent>>,
}

impl RequestResponder {
  pub(crate) fn new(sender: Option<Sender<MessageContent>>) -> Self {
    Self {
      sender,
    }
    
  }
  pub fn respond(&mut self, response: MessageContent) -> Result<(), String> {
    match self.sender.take() {
      None => Err("Already responded".to_string()),
      Some(s) => {
	s.send(response).unwrap();
	Ok(())
      },
    }
  }
}
