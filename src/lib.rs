extern crate termion;

pub mod command_interpreter {

    use std::io::prelude::Read;
    use std::fmt::Write;

    use crate::models::{ContentBuffer, Command};

    pub fn await_command<R: Read>(command: &mut Command, input: &mut R) -> Result<(), std::io::Error> {
        match input.read_exact(&mut command.raw_input) {
            Err(e) => Err(e),
            _ => Ok(()),
        }
    }

    pub fn process_command(command: &Command, buffer: &mut ContentBuffer) -> Option<u16> {
        if command.raw_input.len() != 1 {
            panic!("Raw input should only ever be len 1 for now");
        }

        let cmd = command.raw_input[0];

        match cmd {
            //exit the program
            27 => { return Some(0);}
            //backspace
            127 => { buffer.content.remove(buffer.content.len()-1); },
            //lowercase
            97..=122 => write!(buffer.content, "{}", cmd as char).unwrap(),
            //uppercase
            65..=90 => write!(buffer.content, "{}", cmd as char).unwrap(),
            //ok turns out we want to do this by default right now
            _ => write!(buffer.content, "{}", cmd as char).unwrap(),
        }

        None
    }
}

pub mod display {

    use std::io::*;
    use termion::{clear, cursor};
    use crate::models::{ContentBuffer};

    /* Open question: Should performance be a
     * display concern? Or a caller's concern? Eventually we'll want to do diff
     * analysis and only change what we need*/
    pub fn display_buffer<W: Write>(buffer: &ContentBuffer, screen: &mut W) {
        write!(screen, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
        write!(screen, "{}", buffer.content).unwrap();
        screen.flush().unwrap();
    }
}

pub mod models {
    pub struct ContentBuffer {
        pub content: String, //todo do I want a byte array here?
        pub point: u64,
    }

    pub struct Command {
        pub raw_input: [u8; 1],
    }
}
