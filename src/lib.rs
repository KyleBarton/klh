extern crate termion;

pub mod command_executor {
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
            Command::BufferDelete => {
                write!(log, "Keystroke Processed: <BACKSPACE>\n").unwrap();
                buffer.content.remove(
                    //point actually hangs out at buffer.len (out of range) right now
                    min(buffer.point.try_into().unwrap(), buffer.content.len()-1)
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
}

pub mod input_handler {

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
                        ControlType::CursorBack
                    ))
                }
                Key::Right => {
                    Ok(InputType::Control(
                        ControlType::CursorForward
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
        input_command_map.insert(InputType::Control(ControlType::CursorForward), Command::AdvancePoint);
        input_command_map.insert(InputType::Control(ControlType::CursorBack), Command::RetreatPoint);
        //
        write!(log, "Input Type is {:?}\n", &input_type).unwrap();

        match input_command_map.get(&input_type) {
            Some(cmd) => Ok(*cmd),
            None => Ok(Command::Default),
        }
    }
}

pub mod display {

    use std::io::*;
    use std::cmp::max;
    use std::convert::TryInto;
    use termion::{clear, cursor};
    use crate::models::{ContentBuffer};

    /* Open question: Should performance be a
     * display concern? Or a caller's concern? Eventually we'll want to do diff
     * analysis and only change what we need*/
    pub fn display_buffer<W: Write>(buffer: &ContentBuffer, screen: &mut W) {
        write!(screen, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
        //TODO handling carriage returns is going to have to be a display
        // concern. Perhaps buffer can provide some data that will make it
        // easier though
        //1-based
        let cursor = calculate_cursor(&buffer);
        write!(screen, "{}{}", buffer.content, cursor::Goto(cursor.0, cursor.1)).unwrap();
        screen.flush().unwrap();
    }

    /*indexes are still a little weird here For instance, cursor doesn't seem to
    calculate quite right on delete of the enter key This needs a fully tested
    suite at this point though
     */
    pub fn calculate_cursor(buffer: &ContentBuffer) -> (u16, u16) {
        let point_as_index = buffer.point as usize;
        let before_point = &buffer.content[..point_as_index];

        let y = before_point.matches("\n").count();

        let last_newline = match before_point.rfind('\n') {
            Some(ind) => ind+1,
            None => 0,
        };
        let x = max(
            before_point[last_newline..].len() as i16,
            0
        );

        ((x+1).try_into().unwrap(), (y+1).try_into().unwrap())
    }
}

pub mod models {
    pub struct ContentBuffer {
        pub content: String, //todo do I want a byte array here?
        pub point: i64, //Allowing negatives to keep the program from crashing. Point needs to be more thoroughly thought out
    }

    /*
    Just `Editor` for now, but later can apply to display modifications, project management, etc
    This may be better of as a string that gets looked up later for functions. Or something better.
    TODO Dynamic function lookup in rust
     */
    #[derive(Copy, Clone, Debug)]
    pub enum Command {
        BufferInsert(char),
        BufferDelete,
        AdvancePoint,
        RetreatPoint,
        Quit,
        Default, //mostly for stubbing
    }

    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum ControlType {
        Escape,
        Backspace,
        CursorBack,
        CursorForward,
        CursorUp,
        CursorDown,
    }
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum InputType {
        Waiting, //Initialized, not yet received user input
        Insert(char),
        Control(ControlType),
    }
}
