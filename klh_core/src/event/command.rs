use super::{EventMessage, EventType};

// command.rs

pub struct Command {
  event_type: EventType,
  content: String,
}

impl Command {
  pub fn from_id(
    id: &str,
    content: String,
  ) -> Self {
    Self {
      event_type: EventType::command_from_str(id),
      content,
    }
  }

  pub fn get_event_message(&mut self) -> Result<EventMessage, String> {
    Ok(EventMessage::new(
      self.event_type,
      None,
      Some(self.content.clone()),
    ))
  }
}
