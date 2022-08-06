use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub enum Event {
  Command {
    id: String,
    data: CommandData,
  },
  Query {
    id: String,
  },
}

// TODO needed? clean up
impl Event {
  pub fn from(event: &Event) -> Self {
    match event {
      Event::Command { id, data } => Event::Command { id: id.clone(), data: data.clone() },
      Event::Query { id } => Event::Query { id: id.clone(), },
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

impl PartialEq for Event {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Event::Command {
        id,
        data: _,
      } => {
	match other {
	  Event::Query {
	    id: _, } => false,
	  Event::Command {
	    id: other_id,
	    data: _,
	  } => id == other_id
	}
      },
      Event::Query {
	id,
      } => {
	match other {
	  Event::Command {
	    id: _,
	    data: _
	  } => false,
	  Event::Query {
	    id: other_id,
	  } => id == other_id
	}
      },
    }
  }
}

impl Eq for Event {}

impl Hash for Event {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Event::Command { id, data: _,} => id.hash(state),
      Event::Query { id, } => id.hash(state),
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
