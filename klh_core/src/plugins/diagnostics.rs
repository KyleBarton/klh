use std::{thread, time};

use crate::{plugin::Plugin, event::{EventType, EventMessage, Request, MessageContent}, session::SessionClient};

// TODO need a better way to do this
static COMMAND_EVENT_TYPE_IDS : [&str; 2] = [
  "diagnostics::log_event",
  "diagnostics::slow_bomb",
];

pub(crate) struct Diagnostics {
  event_types: Vec<EventType>,
  session_client: Option<SessionClient>,
}

impl Diagnostics {
  pub(crate) fn new() -> Self {

    //TODO ugly place for this
    let mut event_types: Vec<EventType> = Vec::new();

    for id in COMMAND_EVENT_TYPE_IDS {
      event_types.push(EventType::command_from_str(id));
    }

    Diagnostics{
      event_types,
      session_client: None,
    }
  }
}

pub fn new_log_event() -> Request {
  Request::new(EventType::command_from_str("diagnostics::log_event"), MessageContent::empty())
}

pub fn new_slow_bomb() -> Request {
  Request::new(EventType::command_from_str("diagnostics::slow_bomb"), MessageContent::empty())
}

impl Plugin for Diagnostics {

  fn accept_event(&mut self, event_message: EventMessage) -> Result<(), String> {
    println!("[DIAGNOSTICS] Diagnostics received event");
    match event_message.get_event_type() {
      EventType::Query(_) => {
	println!("[DIAGNOSTICS] don't know this event");
	Ok(())
      },
      EventType::Command(id) => {
	if &id[0.."diagnostics::log_event".len()] == "diagnostics::log_event".as_bytes() {
	  println!("[DIAGNOSTICS] Diagnostics plugin received a (new) log event.");
	}
	if &id[0.."diagnostics::slow_bomb".len()] == "diagnostics::slow_bomb".as_bytes() {
	  println!("[DIAGNOSTICS] Diagnostics processing a (new) slow bomb for 10 seconds.");
	  thread::sleep(time::Duration::from_secs(10));
	  println!("Finished waiting for 10 seconds");
	}
	Ok(())
      }
    }
  }

  fn list_event_types(&self) -> Vec<EventType> {
    self.event_types.clone()
  }

  // TODO do I actually need the client for diagnostics?
  fn receive_client(&mut self, session_client: SessionClient) {
    self.session_client = Some(session_client);
  }
}

