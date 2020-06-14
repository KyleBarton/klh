extern crate termion;
extern crate termios;
extern crate libc;

use std::io;
use termios::*;
use termion::screen::AlternateScreen;
use std::fmt::Write;
use std::io::Write as Otherwise;
use std::fs::File;

use klh::command_interpreter::*;
use klh::display::*;
use klh::models::{ContentBuffer, Command};

fn main() -> io::Result<()> {
    let std_fd = libc::STDIN_FILENO;
    let termios = enable_canononical(std_fd);
    let mut reader = std::io::stdin();
    let mut log = String::new();

    let mut current_buffer = ContentBuffer {
        content: String::new(),
        point: 0,
    };

    let mut screen = AlternateScreen::from(std::io::stdout());

    loop {

        display_buffer(&current_buffer, &mut screen);

        let mut command = Command {
            raw_input: [0;1],
        };

        //TODO to be safe we should be injecting the fd here
        await_command(&mut command, &mut reader).unwrap();

        write!(
            &mut log,
            "Command entered was {:?} ===> {}\n",
            command.raw_input[0],
            command.raw_input[0] as char).unwrap();

        match process_command(&command, &mut current_buffer) {
            Some(_exit_code) => break, //just leave the loop for now
            None => (), //keep looping
        }
    }

    let log_filename = "klh.log";
    save_log(&log_filename, &log)?;

    println!("\nexiting! Log of this session can be found in {}", log_filename);

    disable_canonical(termios, std_fd);

    Ok(())
}


fn enable_canononical(fd: i32) -> termios::Termios {
    let termios = Termios::from_fd(fd).unwrap();
    let mut my_termios = termios.clone();
    my_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(fd, TCSANOW, &mut my_termios).unwrap();
    return termios;
}

fn disable_canonical(term: Termios, fd: i32) {
    tcsetattr(fd, TCSANOW, & term).unwrap();
}

fn save_log(filename: &str, log: &String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    write!(&mut file, "{}", &log).unwrap();
    Ok(())
}


