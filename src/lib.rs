extern crate termion;

//TODO this needs to be user_input or something. These are keystrokes not commands
pub mod command_interpreter {

    use std::convert::TryInto;
    use std::io::prelude::Read;
    use std::fmt::Write;
    use std::cmp::{max, min};
    use termion::input::TermRead;
    use termion::event::Key;

    use crate::models::{ContentBuffer, UserInput, InputType, ControlType};

    pub fn await_input<R: TermRead + Read>(user_input: &mut UserInput, input: &mut R) -> Result<(), std::io::Error> {
        for keystroke in input.keys() {
            match keystroke.unwrap() {
                Key::Backspace => {
                    user_input.input_type = InputType::Control(
                        ControlType::Backspace
                    );
                },
                Key::Up => {
                    user_input.input_type = InputType::Control(
                        ControlType::CursorUp
                    );
                },
                Key::Down => {
                    user_input.input_type = InputType::Control(
                        ControlType::CursorDown
                    );
                },
                Key::Left => {
                    user_input.input_type = InputType::Control(
                        ControlType::CursorBack
                    );
                }
                Key::Right => {
                    user_input.input_type = InputType::Control(
                        ControlType::CursorForward
                    )
                },
                Key::Esc => {
                    user_input.input_type = InputType::Control(
                        ControlType::Escape
                    );
                },
                Key::Char(ch) => {
                    user_input.input_type = InputType::Insert(ch);
                },
                _ => (), //TODO obviously
            }
            //one command at a time rn
            break;
        }
        Ok(())
    }

    //TODO we should really change the nomenclature: these are user-inputs. They need to be translated to commands for the Command_Interpreter
    //By then, this should be constructing an _actual_ command, rather than interacting with buffers
    pub fn process_input<W: Write>(command: &UserInput, buffer: &mut ContentBuffer, log: &mut W) -> Option<u16>{
        match &command.input_type {
            InputType::Control(ctrl_type) => {
                match ctrl_type {
                    ControlType::Backspace => {
                        write!(log, "Keystroke Processed: <BACKSPACE>\n").unwrap();
                        buffer.content.remove(
                            //point actually hangs out at buffer.len (out of range) right now
                            min(buffer.point.try_into().unwrap(), buffer.content.len()-1)
                        );
                        buffer.point = max(buffer.point-1, 0);
                        write!(log, "Point is at: {}\n", buffer.point).unwrap();
                    },
                    ControlType::Escape => {
                        write!(log, "Keystroke Processed: <ESC>\n").unwrap();
                        write!(log, "Point is at: {}\n", buffer.point).unwrap();
                        return Some(0)
                    },
                    ControlType::CursorBack => {
                        write!(log, "Keystroke Processed: <BACK>\n").unwrap();
                        buffer.point = max(0, buffer.point-1);
                        write!(log, "Point is at: {}\n", buffer.point).unwrap();
                    },
                    ControlType::CursorForward => {
                        write!(log, "Keystroke Processed: <FORWARD>\n").unwrap();
                        buffer.point = min(buffer.point+1, buffer.content.len().try_into().unwrap());
                        write!(log, "Point is at {}\n", buffer.point).unwrap();
                    },
                    _ => (),
                }
            }
            InputType::Insert(ch) => {
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
            _ => (),
        };
        None
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

    pub fn calculate_cursor(buffer: &ContentBuffer) -> (u16, u16) {
        let y = buffer.content.matches("\n").count();
        let last_newline = match buffer.content.rfind('\n') {
            Some(ind) => ind,
            None => 0,
        };
        let x = max(
            buffer.content[last_newline..].len() as i16 -1,
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

    pub struct UserInput {
        pub input_type: InputType,
    }

    pub enum ControlType {
        Escape,
        Backspace,
        CursorBack,
        CursorForward,
        CursorUp,
        CursorDown,
    }
    pub enum InputType {
        Waiting, //Initialized, not yet received user input
        Insert(char),
        Control(ControlType),
    }
}
