use std::io;
use std::fs::File;
use std::io::Write;
use crate::display;
use crate::input_handler;
use crate::startup;
use crate::buffer;
use crate::buffer_provider;
use crate::command_executor;
use crate::models::Command;

use termion::screen;

pub struct Session {
    startup_args: startup::StartupArgs,
    current_buffer: buffer::LineBuffer,
    //screen: screen::AlternateScreen, //todo we need a "display member of some sort instead of running this in "run"
}

impl Session {
    //TODO this is where you would new-up the buffer store
    //TODO This actually maybe should be distinct from start_up()
    pub fn new(args: startup::StartupArgs) -> Session {
        let new_buffer = match args.get_file_name() {
            None => buffer_provider::new(buffer_provider::BufferType::Normal).unwrap(),
            Some(f) => buffer_provider::from_file(buffer_provider::BufferType::Normal, f).unwrap(),
            };
        Session {
            startup_args: args,
            current_buffer: new_buffer,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut screen = screen::AlternateScreen::from(std::io::stdout());
        let mut reader = std::io::stdin();
        let mut log = String::new();
        loop {

            display::display_buffer_v2(&self.current_buffer, &mut screen);

            let input = input_handler::await_input_v2(&mut reader, &mut log).unwrap();

            let command: Command = input_handler::process_input_v2(input, &mut log).unwrap();

            match command_executor::execute_command_v2(&command, &mut self.current_buffer, &mut log) {
                Some(_exit_code) => break,
                None => (),
            }

        }

        let log_filename = "klh.log";
        Session::save_log(&log_filename, &log).unwrap();

        Ok(())
    }

    fn save_log(filename: &str, log: &String) -> io::Result<()> {
        let mut file = File::create(filename)?;
        write!(&mut file, "{}", &log).unwrap();
        Ok(())
    }
}
