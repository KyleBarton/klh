use std::mem;

/*For now, buffer owns point and content, and will accept higher-level commands.
It may consume a more pure buffer editor later on for future concurrency
concerns: changes to ContentBuffer need to be blocking*/
pub trait Buffer {
    fn append_at_point(&mut self, chars: &str) -> Result<(), String>; //append at least 1 char
    fn delete_at_point(&mut self, deletions: usize) -> Result<(), String>;
    fn get_char_at_index(&self, index: i64) -> Result<char, String>;
    fn get_next_index_for_char(&self, query_char: char) -> Result<i64, String>;
    fn get_point(&self) -> Result<i64, String>;
    fn move_point(&mut self, index: i64) -> Result<(), String>;
    fn get_chars(&self) -> Result<String, String>; //returns all chars in a big string
}


pub struct LineBuffer {
    point: i64, //currently point is horizontal only, i.e. 0 on a new line
    current_line: usize, //todo how to make this private?
    lines: Vec<String>,
}

//TODO this needs to be only accessible from the buffer_manager
impl LineBuffer {
    pub fn new() -> LineBuffer {
        LineBuffer {
            point: 0,
            current_line: 0,
            lines: Vec::new(),
        }
    }

    fn append_char_at_point(&mut self, ch: char) {
        let pt =
            if ch == '\n' {
                self.current_line += 1;
                0
            }
            else {
                match self.lines.get(self.current_line) {
                    None => {
                        self.lines.push(ch.to_string());
                        1
                    },
                    Some(ln) => {
                        let mut line = ln.to_string();
                        line.push(ch);
                        mem::replace(&mut self.lines[self.current_line], line);
                        self.point + 1
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
        Err(String::from("Not yet implemented"))
    }

    fn get_char_at_index(&self, index: i64) -> Result<char, String> {
        Err(String::from("Not yet implemented"))
    }

    fn get_next_index_for_char(&self, query_char: char) -> Result<i64, String> {
        Err(String::from("Not yet implemented"))
    }

    fn get_point(&self) -> Result<i64, String> {
        Ok(self.point)
    }

    fn move_point(&mut self, index: i64) -> Result<(), String> {
        Err(String::from("Not yet implemented"))
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
        buffer.lines.push(String::from("here already. "));
        buffer.append_at_point("Here now too").unwrap();
        assert_eq!(
            buffer.get_chars().unwrap(),
            String::from("here already. Here now too")
        );

    }

    #[test]
    fn line_buffer_should_append_newlines() {
        let mut buffer = LineBuffer::new();
        buffer.lines.push(String::from("here already. "));
        buffer.append_at_point("\n Here now too").unwrap();
        assert_eq!(
            buffer.get_chars().unwrap(),
            String::from("\
here already. 
 Here now too"
            ),
        );
        assert_eq!(buffer.lines.len(), 2);
    }

    #[test]
    fn line_buffer_should_move_point_on_append() {
        let mut buffer = LineBuffer::new();
        buffer.append_at_point("abcd").unwrap();
        assert_eq!(buffer.get_point().unwrap(), 4)
    }
}
