use std::collections::HashMap;
use std::io::prelude::Read;
use std::io::ErrorKind;
use std::fmt::Write;
use termion::input::TermRead;
use termion::event::Key;

use crate::models::*;

const CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz\" +
\"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\" +
\" !@#$%^&*()_-+=\\|[]{}/?.>,<`~;:\n";

pub fn await_input_v2<R: TermRead + Read>(input: &mut R, /*TODO log*/ _log: &mut impl Write)
                                            -> Result<InputType, std::io::Error> {
    for keystroke in input.keys() {
            return match keystroke.unwrap() {
            Key::Backspace => {
                Ok(InputType::Control(ControlType::Backspace))
            },
            Key::Up => {
                Ok(InputType::Control(
                    ControlType::CursorUp
                ))
            },
            Key::Down => {
                Ok(InputType::Control(
                    ControlType::CursorDown
                ))
            },
            Key::Left => {
                Ok(InputType::Control(
                    ControlType::ArrowLeft
                ))
            }
            Key::Right => {
                Ok(InputType::Control(
                    ControlType::ArrowRight
                ))
            },
            Key::Esc => {
                Ok(InputType::Control(
                    ControlType::Escape
                ))
            },
            Key::Char(ch) => {
                Ok(InputType::Insert(ch))
            },
                _ => Ok(InputType::Waiting), //TODO obviously
        }
    }
    Err(std::io::Error::new(ErrorKind::Other, "Bad keystroke I guess"))
}

//making a decision here that input_handler does not have buffer awareness
// -> it simply processes an input and translates it to a command for
// someone. Buffer_Manager, or some other abstraction, is going to have to
// decide where it goes.
pub fn process_input_v2<W: Write>(input_type: InputType, log: &mut W) -> Result<Command, std::io::Error> {
    //TODO This has to be passed in, and the map can change based on the current state of the editor
    let mut input_command_map: HashMap<InputType, Command> = HashMap::new();

    //TODO uggggh
    for c in CHARS.chars() {
        input_command_map.insert(
            InputType::Insert(c),
            Command::BufferInsert(c)
        );
    }
    input_command_map.insert(InputType::Control(ControlType::Escape), Command::Quit);
    input_command_map.insert(InputType::Control(ControlType::Backspace), Command::BufferDelete);
    input_command_map.insert(InputType::Control(ControlType::ArrowRight), Command::AdvancePoint);
    input_command_map.insert(InputType::Control(ControlType::ArrowLeft), Command::RetreatPoint);
    //
    write!(log, "Input Type is {:?}\n", &input_type).unwrap();

    match input_command_map.get(&input_type) {
        Some(cmd) => Ok(*cmd),
        None => Ok(Command::Default),
    }
}
