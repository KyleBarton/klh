extern crate termios;
extern crate libc;

use std::io;
use std::io::prelude::*;
use termios::*;
use std::fmt::Write;
use std::io::Write as Otherwise;
use std::fs::File;

//backspace == ascii 127
fn main() -> io::Result<()> {
    let std_fd = libc::STDIN_FILENO;
    let termios = enable_canononical(std_fd);
    println!("Begin typing! You're in klh");

    let mut log = String::new();

    //TODO string is not the struc we want to use here
    let mut ed_buffer = String::new();

    loop {
        //TODO to be safe we should be injecting the fd here
        let mut command_buffer = [0;1];
        await_command(&mut command_buffer).unwrap();
        if command_buffer[0] == 'Q' as u8 {
            break;
        }

        process_command(&command_buffer, &mut ed_buffer).unwrap();

        write!(
            &mut log,
            "Command entered was {:?} ===> {}\n",
            command_buffer[0],
            command_buffer[0] as char).unwrap();

        display(&command_buffer);
        //display_buffer(&ed_buffer);
    }

    let log_filename = "klh.log";
    save_log(&log_filename, &log)?;

    println!("\nexiting! Log of this session can be found in {}", log_filename);

    disable_canonical(termios, std_fd);

    Ok(())
}

/*So this has the obvious problem of coupling to command interpretation
That is, display() implicitly has the "echo" action on all commands. What's nice here is we aren't redrawing
every time.
*/
fn display(buffer: &[u8]) {
    if buffer.len() != 1 {
       panic!("Only accepting single-byte buffers at this time");
    }
    print!("{}", buffer[0] as char);
    let stdout = io::stdout();
    stdout.lock().flush().unwrap();
}

fn display_buffer(buffer: &String) {
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

//TODO we need to wrap std::io entirely here
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

fn process_command(command_buf: &[u8], ed_buf: &mut String) -> Result<(), std::io::Error> {
    if command_buf.len() != 1 {
        panic!("Command buf should only every be len 1");
    }
    let cmd = command_buf[0];

    match cmd {
        //lowercase
        97..=122 => write!(ed_buf, "{}", cmd as char).unwrap(),
        //uppercase
        65..=90 => write!(ed_buf, "{}", cmd as char).unwrap(),
        //ok turns out we want to do this by default right now
        _ => write!(ed_buf, "{}", cmd as char).unwrap(),
    };

    Ok(())
}

