use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::buffer;

use crate::buffer::{ Buffer, LineBuffer };

//TODO str is not what we want here for file option, and we need our own errors
pub fn new_buffer(_file_name: Option<&str>) -> Result<impl buffer::Buffer, String> {
    //Err(String::from("not yet implemented"))
    Ok(buffer::LineBuffer::new())
}

pub enum BufferType {
    Normal,
}
pub fn new(buffer_type: BufferType) -> Result<LineBuffer, String> {
    Ok(LineBuffer::new())
}

pub fn from_file(buffer_type: BufferType, file_name: &str) -> Result<LineBuffer, String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(LineBuffer::with_content(&contents, &file_name))
}

pub fn save(buffer: &impl Buffer, log: &mut impl std::fmt::Write) -> Result<(), String> {
    let mut file = match OpenOptions::new().write(true).open(buffer.get_name().unwrap()) {
        Ok(fi) => fi,
        Err(e) => { write!(log, "Problem trying to open file, {}\n", e); return Ok(());},
    };
    let content = &buffer.get_chars().unwrap();
    let content_str = content.as_bytes();

    match file.write(content_str) {
        Err(e) => write!(log, "Problem with writing; {}\n", e).unwrap(),
        Ok(_) => (),
    }

    Ok(())
}
