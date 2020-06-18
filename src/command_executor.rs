use std::cmp::{max, min};
use std::fmt::Write;
use std::convert::TryInto;
use crate::models::*;

//todo logging is all off
pub fn execute_command(command: &Command, buffer: &mut ContentBuffer, log: &mut impl Write) -> Option<u16> {
    write!(log, "Command is {:?}\n", command).unwrap();
    match command {
        Command::BufferInsert(ch) => {
            write!(log, "Keystroke Processed: {}, {:?}\n", ch, *ch as u8).unwrap();

            if buffer.point == buffer.content.len().try_into().unwrap() {
                buffer.content.push(*ch);
            }
            else {
                buffer.content.insert(buffer.point.try_into().unwrap(), *ch);
            }

            buffer.point = min(
                buffer.point + 1,
                buffer.content.len().try_into().unwrap()
            );

            write!(log, "Point is at: {}\n", buffer.point).unwrap();
        },
        Command::Quit => {
            write!(log, "Keystroke Processed: <ESC>\n").unwrap();
            write!(log, "Point is at: {}\n", buffer.point).unwrap();
            return Some(0)
        },
        //TODO this is not performing correctly when point is not at the end of the buffer
        Command::BufferDelete => {
            write!(log, "Keystroke Processed: <BACKSPACE>\n").unwrap();
            buffer.content.remove(
                //point actually hangs out at buffer.len (out of range) right now
                //TODO when Buffer_Editor gets implemented, we need to model point as a position _between_ chars
                min((buffer.point-1).try_into().unwrap(), buffer.content.len()-1)
            );
            buffer.point = max(buffer.point-1, 0);
            write!(log, "Point is at: {}\n", buffer.point).unwrap();
        },
        Command::AdvancePoint => {
            write!(log, "Keystroke Processed: <FORWARD>\n").unwrap();
            buffer.point = min(buffer.point+1, buffer.content.len().try_into().unwrap());
            write!(log, "Point is at {}\n", buffer.point).unwrap();
        },
        Command::RetreatPoint => {
            write!(log, "Keystroke Processed: <BACK>\n").unwrap();
            buffer.point = max(0, buffer.point-1);
            write!(log, "Point is at: {}\n", buffer.point).unwrap();
        }
        _ => (),
    }
    None
}
