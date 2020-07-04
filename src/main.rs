use std::io;
use termios::*;
use termion::screen::AlternateScreen;
use std::io::Write;
use std::fs::File;
use std::env;

use klh::command_executor::*;
use klh::input_handler::*;
use klh::display::*;
use klh::models::Command;
use klh::buffer_provider;

fn main() -> io::Result<()> {
    let std_fd = libc::STDIN_FILENO;
    /*These two lines have to stay together*/
    let given_termios = Termios::from_fd(std_fd).unwrap();
    let mut new_termios = given_termios.clone();
    set_term_raw(&mut new_termios, std_fd).unwrap();
    let mut reader = std::io::stdin();
    /*END*/

    //ARGS
    let mut cli_args: env::Args = env::args();
    cli_args.next();

    let file_name: Option<String> = cli_args.next();
    //END
    let mut log = String::new();

    let mut current_buffer = match file_name {
        None => buffer_provider::new(buffer_provider::BufferType::Normal).unwrap(),
        Some(f) => buffer_provider::from_file(buffer_provider::BufferType::Normal, &f).unwrap(),
    };

    let mut screen = AlternateScreen::from(std::io::stdout());

    loop {

        display_buffer_v2(&current_buffer, &mut screen);

        let input = await_input_v2(&mut reader, &mut log).unwrap();

        let command: Command = process_input_v2(input, &mut log)?;

        match execute_command_v2(&command, &mut current_buffer, &mut log) {
            Some(_exit_code) => break,
            None => (),
        }
    }

    reset_term(given_termios, std_fd);

    let log_filename = "klh.log";
    save_log(&log_filename, &log)?;

    println!("\nexiting! Log of this session can be found in {}", log_filename);


    Ok(())
}


fn set_term_raw(mut term: &mut Termios, fd: i32) -> Result<(), String> {
    termios::cfmakeraw(&mut term);
    //we're not doing anything fancy with display yet, we just need raw input
    term.c_oflag |= OPOST;
    Ok(tcsetattr(fd, TCSANOW, &term).unwrap())
}

fn reset_term(term: Termios, fd: i32) {
    tcsetattr(fd, TCSANOW, &term).unwrap();
}

fn save_log(filename: &str, log: &String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    write!(&mut file, "{}", &log).unwrap();
    Ok(())
}
