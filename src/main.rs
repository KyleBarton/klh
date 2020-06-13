extern crate termion;
extern crate termios;
extern crate libc;

use std::io;
use std::io::prelude::*;
use termios::*;
use termion::*;
use std::fmt::Write;
use std::io::Write as Otherwise;
use std::fs::File;

fn main() -> io::Result<()> {
    let std_fd = libc::STDIN_FILENO;
    let termios = enable_canononical(std_fd);
    let mut log = String::new();

    //TODO string is not the struc we want to use here
    let mut ed_buffer = String::new();

    loop {
        display_buffer(&ed_buffer);

        //TODO to be safe we should be injecting the fd here
        let mut command_buffer = [0;1];
        await_command(&mut command_buffer).unwrap();

        write!(
            &mut log,
            "Command entered was {:?} ===> {}\n",
            command_buffer[0],
            command_buffer[0] as char).unwrap();

        match process_command(&command_buffer, &mut ed_buffer) {
            Some(_exit_code) => break, //just leave the loop for now
            None => (), //keep looping
        };


    }

    let log_filename = "klh.log";
    save_log(&log_filename, &log)?;

    println!("\nexiting! Log of this session can be found in {}", log_filename);

    disable_canonical(termios, std_fd);

    Ok(())
}

/* Open question: Should performance be a
 * display concern? Or a caller's concern? Eventually we'll want to do diff
 * analysis and only change what we need*/
fn display_buffer(buffer: &String) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    print!("{}", buffer);
    io::stdout().lock().flush().unwrap();
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

/*
TODO We need to be abstract to the input buffer here. Impls of reader seem
reasonable to start
 */
fn await_command(buffer: &mut[u8]) -> Result<(), std::io::Error> {
    let mut reader = io::stdin();
    match reader.read_exact(buffer) {
        Err(e) => Err(e),
        _ => Ok(())
    }
}

fn save_log(filename: &str, log: &String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    write!(&mut file, "{}", &log).unwrap();
    Ok(())
}

fn process_command(command_buf: &[u8], ed_buf: &mut String) -> Option<u16> {
    if command_buf.len() != 1 {
        panic!("Command buf should only every be len 1");
    }
    let cmd = command_buf[0];

    //TODO the arrow keys make esp happen right now, we need to fix that
    match cmd {
        //exit the program
        27 => { return Some(0);}
        //backspace
        127 => { ed_buf.remove(ed_buf.len()-1); },
        //lowercase
        97..=122 => write!(ed_buf, "{}", cmd as char).unwrap(),
        //uppercase
        65..=90 => write!(ed_buf, "{}", cmd as char).unwrap(),
        //ok turns out we want to do this by default right now
        _ => write!(ed_buf, "{}", cmd as char).unwrap(),
    };
    None
}

