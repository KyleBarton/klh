use std::{thread, time};

use crate::{plugin::Plugin, event::{EventType, EventMessage}, session::SessionClient};

pub(crate) struct Diagnostics {
  event_types: Vec<EventType>,
  session_client: Option<SessionClient>,
}

impl Diagnostics {
  pub(crate) fn new() -> Self {

    //TODO ugly place for this
    let mut event_types: Vec<EventType> = Vec::new();

    event_types.push(EventType::command_from_str("diagnostics::log_event"));
    event_types.push(EventType::command_from_str("diagnostics::slow_bomb"));

    Diagnostics{
      event_types,
      session_client: None,
    }
  }
}

impl Plugin for Diagnostics {

  fn accept_event_v2(&mut self, event_message: EventMessage) -> Result<(), String> {
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

