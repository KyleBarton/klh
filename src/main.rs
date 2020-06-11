extern crate termios;
extern crate libc;

use std::env;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;
use termios::*;

fn main() -> io::Result<()> {
    let std_fd = libc::STDIN_FILENO;
    println!("Enter a single character: ");
    let termios = Termios::from_fd(std_fd).unwrap();
    let mut my_termios = termios.clone();
    my_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(std_fd, TCSANOW, &mut my_termios).unwrap();

    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    println!("Hit a key!");
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    println!("You have hit: {:?}", buffer[0] as char);
    tcsetattr(std_fd, TCSANOW, & termios).unwrap();
    Ok(())
}
    // loop {
    //     let mut buffer = [0;1];
    //     let stdin = io::stdin();
    //     let mut handle = stdin.lock();
    //     handle.read_exact(&mut buffer);
    //     //io::stdin().read_exact(&mut buffer)?;
    //     print!("{:?}", buffer[0]);
    //     io::stdout().flush();
    //     if buffer[0] == 113 {
    //         break;
    //     }
    //     text.push(buffer[0] as char);
    // }
    //     let input: i32 = std::io::stdin()
    //     .bytes()
    //     .next()
    //     .and_then(|result| { result.ok() })
    //     .map(|byte| byte as i32)
    //     .unwrap();
    // println!("Char entered is {:?}", input);

    // print!("String entered is {} \n", text);

    // let mut buffer = [0;1];
    // //let mut handle = stdin.lock();
    // //handle.read_to_string(&mut buffer)?;
    // io::stdout().flush();

    // // io::stdin()
    // //     .read_line(&mut buffer)
    // //     .expect("Failure to read line");
    // // print!("$> {}", &buffer);
    // print!("Text entered is {:?} \n", buffer);
// use std::io::Read;

// let input: Option<i32> = std::io::stdin()
//     .bytes() 
//     .next()
//     .and_then(|result| result.ok())
//     .map(|byte| byte as i32);

// println!("{:?}", input);
// fn main() {
//     let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
//     // on /dev/stdin or /dev/tty
//     let termios = Termios::from_fd(stdin).unwrap();
//     let mut new_termios = termios.clone();  // make a mutable copy of termios 
//     // that we will modify
//     new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
//     tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
//     let stdout = io::stdout();
//     let mut reader = io::stdin();
//     let mut buffer = [0;1];  // read exactly one byte
//     print!("Hit a key! ");
//     stdout.lock().flush().unwrap();
//     reader.read_exact(&mut buffer).unwrap();
//     println!("You have hit: {:?}", buffer);
//     tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to 
//     // original termios data
// }


// use std::io::{self, Read};

// fn main() -> io::Result<()> {
//     let mut buffer = String::new();
//     let stdin = io::stdin();
//     let mut handle = stdin.lock();

//     handle.read_to_string(&mut buffer)?;
//     Ok(())
// }
// fn main_old() -> Result<(), io::Error> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         panic!("Can only take one input file as a parameter!");
//     }
//     // Ok this is fine, now let's actually open into a buffer
//     let file_name = &args[1];
//     let path = Path::new(&file_name);
//     let display = path.display();
//     let mut file = match File::open(&path) {
//         Err(why) => panic!("Couldn't open file because {}!", why.to_string()),
//         Ok(file) => file
//     };
//     let mut contents = String::new();
// 	// Welp, anything past this is gonna need a view
//     match file.read_to_string(&mut contents) {
//         Err(why) => panic!("couldn't read {}: {}", display,
//                                                    why.to_string()),
//         Ok(_) => print!("Nice, sending the contents of {} to the view", display),
//     };
//     //view work
//     let stdout = io::stdout().into_raw_mode()?;
//     let backend = TermionBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     terminal.draw(|mut f| {
//          let chunks = Layout::default()
//             .direction(Direction::Vertical)
//             .margin(1)
//             .constraints(
//                 [
//                     Constraint::Percentage(10),
//                     Constraint::Percentage(80),
//                     Constraint::Percentage(10)
//                 ].as_ref()
//             )
//             .split(f.size());
//         let block = Block::default()
//              .title("Block")
//              .borders(Borders::ALL);
//         f.render_widget(block, chunks[0]);
//         let block = Block::default()
//              .title("Block 2")
//              .borders(Borders::ALL);
//         f.render_widget(block, chunks[1]);
//         print!("This is a bit of std output that is entered after rendering has occured lets see what happens");
//     })
// }

