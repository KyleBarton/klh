extern crate rustbox;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rustbox::{Color, RustBox};
use rustbox::Key;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Can only take one input file as a parameter!");
    }
    // Ok this is fine, now let's actually open into a buffer
    let file_name = &args[1];
	let path = Path::new(&file_name);
	let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file because {}!", why.to_string()),
        Ok(file) => file
    };
    let mut s = String::new();
	// Welp, anything past this is gonna need a view
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.to_string()),
        Ok(_) => print!("Nice, sending the contents of {} to the view", display),
    };
    //view work
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, &s);
    //rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
     //             "Press 'q' to quit.");
    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.to_string()),
            _ => { }
        }
    }
}

