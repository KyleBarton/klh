use std::{thread, time};

use crate::{plugin::Plugin, event::{Event, CommandData}, dispatch::DispatchClient};

pub(crate) struct Diagnostics {
  events: Vec<Event>,
  dispatch_client: Option<DispatchClient>,
}

impl Diagnostics {
  pub(crate) fn new() -> Self {

    //TODO ugly place for this
    let mut events: Vec<Event> = Vec::new();

    // Log message
    events.push(Event::Command {
      id: String::from("diagnostics::log_event"),
      data: CommandData {
	docs: String::from("Sends a message to Diagnostics plugin")
      }
    });
    // A diagnostic slow bomb to verify async integrity
    events.push(Event::Command {
      id: String::from("diagnostics::slow_bomb"),
      data: CommandData {
	docs: String::from("I really need json()"),
      }
    });

    Diagnostics{
      events,
      dispatch_client: None,
    }
  }
}

impl Plugin for Diagnostics {
    fn accept_event(&self, event: Event) -> Result<(), String> {
      println!("[DIAGNOSTICS] Diagnostics received event {:?}", event);
      match event {
	Event::Command {
	  id,
	  data: _data
	} => {
	  match id.as_str() {
	    "diagnostics::log_event" => {
	      println!("[DIAGNOSTICS] Diagnostics plugin received a log event.");
	      Ok(())
	    }
	    "diagnostics::slow_bomb" => {
	      println!("[DIAGNOSTICS] Diagnostics processing a slow bomb for 10 seconds.");
	      // Ok let's go faster
	      thread::sleep(time::Duration::from_secs(10));
	      println!("Finished waiting for 10 seconds");
	      // TODO all this needs to be replaced
	      // let stuff : String = data.docs.clone();
	      // Ok(())
	      Ok(())
	    }
	    _ => {
	      Ok(())
	    }
	  } 
	},
	Event::Query {
	  id: _,
	} => Err(String::from("No queries defined for plugin")),
      }
    }

  fn list_events(&self) -> Vec<Event> {
    self.events.clone()
  }

  // TODO do I actually need the client for diagnostics?
  fn receive_client(&mut self, dispatch_client: DispatchClient) {
    self.dispatch_client = Some(dispatch_client);
  }
}

