use std::{thread, time};

use log::{debug, warn, info};

use crate::{plugin::Plugin, messaging::{MessageType, Message, MessageContent}, session::SessionClient};


pub mod requests;
pub mod models;

// TODO need a better way to do this
static COMMAND_MESSAGE_TYPE_IDS : [&str; 2] = [
  "diagnostics::log_event",
  "diagnostics::slow_bomb",
];

pub(crate) struct Diagnostics {
  message_types: Vec<MessageType>,
  session_client: Option<SessionClient>,
}

impl Diagnostics {
  pub fn new() -> Self {

    //TODO ugly place for this
    let mut message_types: Vec<MessageType> = Vec::new();

    for id in COMMAND_MESSAGE_TYPE_IDS {
      message_types.push(MessageType::command_from_str(id));
    }

    Diagnostics{
      message_types,
      session_client: None,
    }
  }
}
impl Plugin for Diagnostics {

  fn accept_message(&mut self, mut message: Message) -> Result<(), String> {
    debug!("[DIAGNOSTICS] Diagnostics received message");
    match message.get_message_type() {
      MessageType::Query(_) => {
	warn!("[DIAGNOSTICS] don't know this message");
	Ok(())
      },
      MessageType::Command(id) => {
	if &id[0.."diagnostics::log_event".len()] == "diagnostics::log_event".as_bytes() {
	  info!("[DIAGNOSTICS] Diagnostics plugin received a (new) log event.");
	}
	if &id[0.."diagnostics::slow_bomb".len()] == "diagnostics::slow_bomb".as_bytes() {
	  debug!("[DIAGNOSTICS] Diagnostics processing a (new) slow bomb for 10 seconds.");
	  let content: models::SlowBombContent = message.get_content()
	    .expect("Should have content")
	    .deserialize()
	    .expect("Should have slow bomb content");
	  thread::sleep(time::Duration::from_secs(content.interval_seconds));
	  let mut responder = message.get_responder()
	    .expect("Should not have responded yet.");
	  info!("Finished waiting for 10 seconds");
	  responder.respond(MessageContent::empty()).unwrap();
	}
	Ok(())
      }
    }
  }

  fn list_message_types(&self) -> Vec<MessageType> {
    self.message_types.clone()
  }

  // TODO do I actually need the client for diagnostics?
  fn receive_client(&mut self, session_client: SessionClient) {
    self.session_client = Some(session_client);
  }
}

