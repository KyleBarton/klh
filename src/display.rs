use std::io::*;
use std::cmp::max;
use std::convert::TryInto;
use termion::{clear, cursor};
use crate::models::{ContentBuffer};

/* Open question: Should performance be a
    * display concern? Or a caller's concern? Eventually we'll want to do diff
    * analysis and only change what we need*/
pub fn display_buffer<W: Write>(buffer: &ContentBuffer, screen: &mut W) {
    write!(screen, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
    //TODO handling carriage returns is going to have to be a display
    // concern. Perhaps buffer can provide some data that will make it
    // easier though
    //1-based
    let cursor = calculate_cursor(&buffer);
    write!(screen, "{}{}", buffer.content, cursor::Goto(cursor.0, cursor.1)).unwrap();
    screen.flush().unwrap();
}

/*indexes are still a little weird here For instance, cursor doesn't seem to
calculate quite right on delete of the enter key This needs a fully tested
suite at this point though
    */
pub fn calculate_cursor(buffer: &ContentBuffer) -> (u16, u16) {
    let point_as_index = buffer.point as usize;
    let before_point = &buffer.content[..point_as_index];

    let y = before_point.matches("\n").count();

    let last_newline = match before_point.rfind('\n') {
        Some(ind) => ind+1,
        None => 0,
    };
    let x = max(
        before_point[last_newline..].len() as i16,
        0
    );

    ((x+1).try_into().unwrap(), (y+1).try_into().unwrap())
}
