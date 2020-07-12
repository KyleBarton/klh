use crate::buffer;
use crate::buffer_provider;
use crate::models::Command;
use log::info;

pub fn execute_command_v2(command: &Command, buffer: &mut impl buffer::Buffer) -> Option<u16> {
  info!("Executing command: {:?}", command);
  match command {
    Command::BufferInsert(ch) => {
      buffer.append_at_point(&ch.to_string()).unwrap();

      info!("Inserted: {}, {:?}", ch, *ch as u8);
    }
    Command::Quit => {
      info!("Exiting command executor on quit command");
      return Some(0);
    }
    Command::BufferDelete => {
      buffer.delete_at_point(1).unwrap();
      info!("Executed delete at point");
    }
    Command::AdvancePoint => {
      buffer.move_current_location(1).unwrap();
      info!("Advanced point");
    }
    Command::RetreatPoint => {
      buffer.move_current_location(-1).unwrap();
      info!("Retreated point");
    }
    Command::Save => {
      buffer_provider::save(buffer).unwrap();
      info!("Successfully saved to {}", &buffer.get_name().unwrap());
    }
    _ => {
      info!("Command not handled, skipping execution");
      ()
    }
  }
  info!(
    "Command processed, point is at {:?}",
    buffer.get_current_location().unwrap()
  );
  None
}
