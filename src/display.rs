use std::io::*;
use std::convert::TryInto;
use termion::{clear, cursor};
use crate::buffer;

/* Open question: Should performance be a
    * display concern? Or a caller's concern? Eventually we'll want to do diff
    * analysis and only change what we need*/
pub fn display_buffer_v2(buffer: &impl buffer::Buffer, screen: &mut impl Write) {
    write!(screen, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
    let location = buffer.get_current_location().unwrap();
    let cursor = location.as_tuple();
    write!(
        screen,
        "{}{}",
        buffer.get_chars().unwrap(),
        //cursor is 1-based
        cursor::Goto((cursor.0+1).try_into().unwrap(), (cursor.1+1).try_into().unwrap())
    ).unwrap();
    screen.flush().unwrap();
}
