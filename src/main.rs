use std::io;
use termios::*;
use termion::screen::AlternateScreen;
use std::io::Write;
use std::fs::File;

use klh::command_executor::*;
use klh::input_handler::*;
use klh::display::*;
use klh::models::Command;
use klh::buffer;

fn main() -> io::Result<()> {
    let std_fd = libc::STDIN_FILENO;
    /*These two lines have to stay together*/
    let termios = disable_canonical(std_fd);
    let mut reader = std::io::stdin();
    /*END*/
    let mut log = String::new();

    let mut current_buffer_v2 = buffer::LineBuffer::new();

    let mut screen = AlternateScreen::from(std::io::stdout());

    loop {

        display_buffer_v2(&current_buffer_v2, &mut screen);

        let input = await_input_v2(&mut reader, &mut log).unwrap();

        let command: Command = process_input_v2(input, &mut log)?;

        match execute_command_v2(&command, &mut current_buffer_v2, &mut log) {
            Some(_exit_code) => break,
            None => (),
        }
    }

    let log_filename = "klh.log";
    save_log(&log_filename, &log)?;

    println!("\nexiting! Log of this session can be found in {}", log_filename);

    reenable_canonical(termios, std_fd);

    Ok(())
}


fn disable_canonical(fd: i32) -> termios::Termios {
    let termios = Termios::from_fd(fd).unwrap();
    let mut my_termios = termios.clone();
    my_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(fd, TCSANOW, &mut my_termios).unwrap();
    return termios;
}

fn reenable_canonical(term: Termios, fd: i32) {
    tcsetattr(fd, TCSANOW, & term).unwrap();
}

fn save_log(filename: &str, log: &String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    write!(&mut file, "{}", &log).unwrap();
    Ok(())
}
