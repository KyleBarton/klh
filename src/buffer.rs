use std::mem;

/*For now, buffer owns point and content, and will accept higher-level commands.
It may consume a more pure buffer editor later on for future concurrency
concerns: changes to ContentBuffer need to be blocking*/
//from craft: point can be internal data only. Callers shouldn't need it
// but what about... cursor? or last insert place, and let cursor calculate from there
// index could just be the return val of insert/delete
/*Craft: Point_Move moves the point forward (if count is positive) or backward
 * (if negative) by abs(count) characters.*/
/*Craft on buffer edits:	void Insert_Char(char c);
void Insert_String(char *string);
void Replace_Char(char c);
void Replace_String(char *string);
status Delete(int count);
status Delete_Region(mark_name name);
status Copy_Region(char *buffer_name, mark_name name);*/
pub trait Buffer {
    fn append_at_point(&mut self, chars: &str) -> Result<(), String>; //append at least 1 char
    fn delete_at_point(&mut self, deletions: usize) -> Result<(), String>;
    fn get_char_at_index(&self, index: i64) -> Result<char, String>;
    fn get_next_for_char(&self, query_char: char) -> Result<Coordinate, String>;
    fn get_current_location(&self) -> Result<Coordinate, String>;
    fn move_current_location(&mut self, amount: i64) -> Result<Coordinate, String>;
    fn get_chars(&self) -> Result<String, String>; //returns all chars in a big string
}

pub trait Location {
    fn get_xy_coordinates(&self) -> Result<(usize, usize), String>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinate {
    offset: usize,
    line: usize,
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate {
            line: 0,
            offset: 0
        }
    }
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.offset, self.line)
    }
}


pub struct LineBuffer {
    point: Coordinate, //currently point is horizontal only, i.e. 0 on a new line
    lines: Vec<String>,
}

//TODO this needs to be only accessible from the buffer_manager
impl LineBuffer {
    pub fn new() -> LineBuffer {
        LineBuffer {
            point: Coordinate::new(),
            lines: Vec::new(),
        }
    }

    /*Appends a single character at a point synchronously. We should look to
     * deprecate this method*/
    fn append_char_at_point(&mut self, ch: char) {
        let pt =
            if ch == '\n' {
                if self.point.offset == self.lines[self.point.line].len() {
                    self.lines.insert(self.point.line+1, String::from(""));
                    Coordinate {
                        line: self.point.line + 1,
                        offset: 0,
                    }
                }
                else {
                    let newlines = self.lines[self.point.line].split_at(self.point.offset);
                    //copy to new strings so we can memreplace below
                    let first_line_part = String::from(newlines.0);
                    let second_line_part = String::from(newlines.1);
                    mem::replace(&mut self.lines[self.point.line], first_line_part);
                    self.lines.insert(self.point.line+1, second_line_part);
                    Coordinate {
                        line: self.point.line +1,
                        offset: 0,
                    }
                }
            }
            else {
                match self.lines.get(self.point.line) {
                    None => {
                        self.lines.push(ch.to_string());
                        Coordinate {
                            line: self.point.line,
                            offset: 1,
                        }
                    },
                    Some(ln) => {
                        let mut line = ln.to_string();
                        line.insert(self.point.offset, ch);
                        mem::replace(&mut self.lines[self.point.line], line);
                        Coordinate {
                            line: self.point.line,
                            offset: self.point.offset + 1
                        }
                    },
                }
            };
        self.point = pt;
    }
}

impl Buffer for LineBuffer {
    //TODO error control flow isn't good here
    fn append_at_point(&mut self, chars: &str) -> Result<(), String> {
        chars.chars().for_each(|ch| {
            self.append_char_at_point(ch);
        });
        Ok(())
    }

    fn delete_at_point(&mut self, deletions: usize) -> Result<(), String> {
        for _n in 0..deletions {
            match self.lines[self.point.line].pop() {
                Some(_ch) => self.point.offset =
                if self.point.offset < 1 {
                    0
                } else {
                    self.point.offset -1
                },
                None => {
                    self.point.line =
                        if self.point.line < 1 {
                            0
                        } else {
                            self.lines.remove(self.point.line);
                            self.point.line -1
                    };
                },
            }
        }
        Ok(())
    }

    fn move_current_location(&mut self, amount: i64) -> Result<Coordinate, String> {
        if amount < 0 {
            for _n in amount..0 {
                self.point.offset = {
                    if self.point.offset < 1 {
                        if self.point.line < 1 {
                            0
                        }
                        else {
                            self.point.line = self.point.line - 1;
                            self.lines[self.point.line].len()

                        }
                    } else {
                        self.point.offset - 1
                    }
                };
            }
        }
        else {
            for _n in 0..amount {
                self.point.offset =
                    if self.point.offset == self.lines[self.point.line].len() {
                        if self.point.line < (self.lines.len() -1) {
                            self.point.line = self.point.line + 1;
                            0
                        }
                        else {
                            self.point.offset
                        }
                    }
                    else {
                        self.point.offset + 1
                };
            }
        }
        Ok(self.point)
    }

    fn get_char_at_index(&self, index: i64) -> Result<char, String> {
        Err(String::from("Not yet implemented"))
    }

    fn get_next_for_char(&self, query_char: char) -> Result<Coordinate, String> {
        Err(String::from("Not yet implemented"))
    }

    fn get_current_location(&self) -> Result<Coordinate, String> {
        Ok(self.point)
    }


    fn get_chars(&self) -> Result<String, String> {
        //todo this may be copying when we actually want to lend
        let chars: String = String::from(&self.lines.join("\n"));
        Ok(chars)
        //return Ok(&self.lines.)
    }
}

#[cfg(test)]
mod test {
    use crate::buffer::Coordinate;
use crate::buffer::{Buffer, LineBuffer};

    #[test]
    fn line_buffer_should_append_at_point() {
        let mut buffer: LineBuffer = LineBuffer::new();
        buffer.append_at_point("a").unwrap();
        assert_eq!(buffer.get_chars().unwrap(), String::from("a"))
    }

    #[test]
    fn line_buffer_should_append_many_at_point() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();
        assert_eq!(buffer.get_chars().unwrap(), String::from("abcd"))
    }
    #[test]
    fn line_buffer_should_append_existing_text() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("here already. ").unwrap();
        buffer.append_at_point("Here now too").unwrap();
        assert_eq!(
            buffer.get_chars().unwrap(),
            String::from("here already. Here now too")
        );

    }

    #[test]
    fn line_buffer_should_append_newline() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("here already. ").unwrap();
        buffer.append_at_point("\n Here now too").unwrap();
        assert_eq!(
            buffer.get_chars().unwrap(),
            String::from("\
here already. 
 Here now too"
            ),
        );
        assert_eq!(buffer.lines.len(), 2);
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate{ line: 1, offset: 13})
    }

    #[test]
    fn line_buffer_should_append_consecutive_newlines() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("ab\n\n").unwrap();

        assert_eq!(buffer.get_chars().unwrap(), String::from("ab\n\n"));
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 2, offset: 0})
    }

    #[test]
    fn line_buffer_should_move_point_on_append() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();
        assert_eq!(buffer.get_current_location().unwrap().offset, 4)
    }

    #[test]
    fn line_buffer_should_move_point_across_newlines_on_append() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("ab\nc").unwrap();
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 1, offset: 1})
    }

    #[test]
    fn line_buffer_should_append_in_middle_of_content() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();
        buffer.move_current_location(-2).unwrap();

        buffer.append_at_point("e").unwrap();
        assert_eq!(buffer.get_chars().unwrap(), String::from("abecd"))
    }

    #[test]
    fn line_buffer_should_append_newline_in_midddle_of_content() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();
        buffer.move_current_location(-2).unwrap();

        buffer.append_at_point("\nf").unwrap();

        assert_eq!(buffer.get_chars().unwrap(), String::from("ab\nfcd"));
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 1, offset: 1});
    }

    #[test]
    fn line_buffer_should_delete_char_from_point() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();

        buffer.delete_at_point(1).unwrap();
        assert_eq!(buffer.get_chars().unwrap(), String::from("abc"))
    }

    #[test]
    fn line_buffer_should_delete_some_chars_from_point() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();

        buffer.delete_at_point(2).unwrap();
        assert_eq!(buffer.get_chars().unwrap(), String::from("ab"))
    }

    #[test]
    fn line_buffer_should_delete_newlines() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("ab\ncd").unwrap();

        buffer.delete_at_point(3).unwrap();
        assert_eq!(buffer.get_chars().unwrap(), String::from("ab"))
    }

    #[test]
    fn line_buffer_should_not_delete_past_beginning_of_buffer() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd");

        buffer.delete_at_point(5).unwrap();

        assert_eq!(buffer.get_chars().unwrap(), String::from(""));
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 0, offset: 0,})
    }

    #[test]
    fn line_buffer_should_move_current_location_backward() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();

        buffer.move_current_location(-2).unwrap();
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 0, offset: 2,})
    }

    #[test]
    fn line_buffer_should_not_move_current_location_past_beginning_of_buffer() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();

        buffer.move_current_location(-5).unwrap();
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 0, offset: 0,})
    }

    #[test]
    fn line_buffer_should_move_current_location_forward() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();
        buffer.move_current_location(-2).unwrap();

        buffer.move_current_location(1).unwrap();

        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 0, offset: 3,})
    }

    #[test]
    fn line_buffer_should_not_move_current_location_past_end_of_buffer() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 0, offset: 4,});

        buffer.move_current_location(1).unwrap();

        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 0, offset: 4,})
    }

    #[test]
    fn line_buffer_should_move_current_location_up_a_line() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("ab\ncd").unwrap();
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 1, offset: 2,});

        buffer.move_current_location(-3).unwrap();

        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 0, offset: 2,})
    }

    #[test]
    fn line_buffer_should_move_current_location_down_a_line() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("ab\ncde").unwrap();
        buffer.move_current_location(-4).unwrap();
        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 0, offset: 2});

        buffer.move_current_location(2).unwrap();

        assert_eq!(buffer.get_current_location().unwrap(), Coordinate { line: 1, offset: 1})
    }
}
