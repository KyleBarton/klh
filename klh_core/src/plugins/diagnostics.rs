use crate::{plugin::{Plugin, PluginTransmitter, PluginChannel}, event::{Event, CommandData}, dispatch::DispatchClient};

pub(crate) struct Diagnostics {
  events: Vec<Event>,
  dispatch_client: Option<DispatchClient>,
  plugin_channel: PluginChannel,
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

    Diagnostics{
      events: Vec::new(),
      dispatch_client: None,
      plugin_channel: PluginChannel::new(),
    }
  }
}

impl Plugin for Diagnostics {
    fn accept_event(&self, event: Event) -> Result<(), String> {
      match event {
	Event::Command {
	  id,
	  data
	} => {
	  if (id.eq("diagnostics::log_event")) {
	    Ok(())
	  } else {
	    println!("Diagnostics plugin received a log event.");
	    Ok(())
	  }
	},
	Event::Query {
	  id,
	  plugin_id
	} => Err(String::from("No queries defined for plugin")),
      }
    }

    fn clone_transmitter(&self) -> Result<PluginTransmitter, String> {
        Ok(self.plugin_channel.transmitter.clone())
    }

    fn list_events(&self) -> Vec<Event> {
        self.events.clone()
    }

    fn receive_client(&mut self, dispatch_client: DispatchClient) {
        self.dispatch_client = Some(dispatch_client);
    }
}

