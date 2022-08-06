// mod.rs

use crate::{event::{Event, CommandData, }, dispatch::DispatchClient, plugin::Plugin};

pub(crate) struct Buffers {
  events: Vec<Event>,
  dispatch_client: Option<DispatchClient>,
}

impl Buffers {
  pub(crate) fn new() -> Self {
    let mut events: Vec<Event> = Vec::new();

    events.push(Event::Command {
      id: String::from("buffers::create_buffer"),
      data: CommandData {
	docs: String::from("Creates a buffer with relevant info"),
      }
    });
    events.push(Event::Query {
      id: String::from("buffers::list_buffers"),
    });

    Self {
      events,
      dispatch_client: None,
    }
  }

  // TODO
  // fn handleListBuffersQuery(query: Event) -> Result<(), String> {

    
  //   Ok(())
  // }
}

impl Default for Buffers {
    fn default() -> Self {
        Self::new()
    }
}

// impl Default for Buffers {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Plugin for Buffers {
    fn accept_event(&self, event: Event) -> Result<(), String> {
      // TODO we should clean the logging situation up, and use Diagnostics properly
      println!("[BUFFERS] Buffers plugin received an event: {:?}", event);
      match event {
	Event::Command {
	  id,
	  data: _
	} => {
	  match id.as_str() {
	    "buffers::create_buffer" => {
	      println!("[BUFFERS] Buffers plugin received create buffer command");
	      Ok(())
	    },
	    _ => {
	      Ok(())
	    }
	  }
	},
	Event::Query {
	  id,
	} => {
	  match id.as_str() {
	    "buffers::list_buffers" => {
	      println!("[BUFFERS] Buffers plugin received list buffers query.");

	      Ok(())
	    },
	    _ => Ok(())
	  }
	},
      }
    }

    fn list_events(&self) -> Vec<Event> {
        self.events.clone()
    }

    fn receive_client(&mut self, dispatch_client: DispatchClient) {
        self.dispatch_client = Some(dispatch_client)
    }
}
