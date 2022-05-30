#[derive(Clone, Debug)]
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
  pub fn command_from(message: &str) -> Self {
    Event::Command {
      id: String::from("simple_message"),
      data: CommandData {
	docs: String::from(message)
      }
    }
  }
}


#[derive(Clone, Debug)]
pub struct CommandData {
  docs: String
}


impl CommandData {
  pub fn docs(&self) -> String {
    self.docs.clone()
  }
}
