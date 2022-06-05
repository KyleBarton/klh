#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
  Command {
    id: String,
    data: CommandData,
  },
  Query {
    id: String,
    plugin_id: String,
    // Responder: impl QueryResponder,
  },
}

impl Event {
  pub fn from(event: &Event) -> Self {
    match event {
      Event::Command { id, data } => Event::Command { id: id.clone(), data: data.clone() },
      Event::Query { id, plugin_id } => Event::Query { id: id.clone(), plugin_id: plugin_id.clone() }
    }
  }

  // TODO this seems like the wrong place for this
  pub fn command_from(message: &str) -> Self {
    Event::Command {
      id: String::from("simple_message"),
      data: CommandData {
	docs: String::from(message)
      }
    }
  }
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CommandData {
  pub docs: String
}


impl CommandData {
  pub fn docs(&self) -> String {
    self.docs.clone()
  }
}
