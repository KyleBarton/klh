use std::fmt::Write;
use crate::models::Command;
use crate::buffer;
use crate::buffer_provider;

//TODO logging still
pub fn execute_command_v2(command: &Command, buffer: &mut impl buffer::Buffer, log: &mut impl Write) -> Option<u16> {
    write!(log, "Command is {:?}\n", command).unwrap();
    match command {
        Command::BufferInsert(ch) => {
            write!(log, "Command insert: {}, {:?}\n", ch, *ch as u8).unwrap();

            buffer.append_at_point(&ch.to_string()).unwrap();

            write!(log, "Point is at: {:?}\n", buffer.get_current_location().unwrap()).unwrap();
        },
        Command::Quit => {
            write!(log, "Keystroke Processed: <ESC>\n").unwrap();
            write!(log, "Point is at: {:?}\n", buffer.get_current_location().unwrap()).unwrap();
            return Some(0)
        },
        Command::BufferDelete => {
            write!(log, "Keystroke Processed: <BACKSPACE>\n").unwrap();
            buffer.delete_at_point(1).unwrap();
            write!(log, "Point is at: {:?}\n", buffer.get_current_location().unwrap()).unwrap();
        },
        Command::AdvancePoint => {
            write!(log, "Keystroke Processed: <FORWARD>\n").unwrap();
            buffer.move_current_location(1).unwrap();
            write!(log, "Point is at {:?}\n", buffer.get_current_location().unwrap()).unwrap();
        },
        Command::RetreatPoint => {
            write!(log, "Keystroke Processed: <BACK>\n").unwrap();
            buffer.move_current_location(-1).unwrap();
            write!(log, "Point is at: {:?}\n", buffer.get_current_location().unwrap()).unwrap();
        },
        Command::Save => {
            write!(log, "Command Processed: SAVE\n").unwrap();
            buffer_provider::save(buffer, log).unwrap();
            write!(log, "Successfully saved to {}\n", &buffer.get_name().unwrap());
        }
        _ => (),
    }
    None
}
