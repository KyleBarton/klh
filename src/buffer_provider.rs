use log::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::buffer::{Buffer, LineBuffer};

#[derive(Debug)]
pub enum BufferType {
  Normal,
}

pub fn new(buffer_type: BufferType) -> Result<Box<dyn Buffer>, String> {
  match buffer_type {
    BufferType::Normal => Ok(Box::new(LineBuffer::new())),
  }
}

pub fn from_file(buffer_type: BufferType, file_name: &str) -> Result<Box<dyn Buffer>, String> {
  match buffer_type {
    BufferType::Normal => {
      let mut file = File::open(file_name).unwrap();
      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();
      Ok(Box::new(LineBuffer::with_content(&contents, &file_name)))
    }
  }
}

pub fn save(buffer: &Box<dyn Buffer>) -> Result<(), String> {
  let mut file = match OpenOptions::new()
    .write(true)
    .open(buffer.get_name().unwrap())
  {
    Ok(fi) => fi,
    Err(e) => {
      //TODO handle at caller not here
      error!("Problem trying to open file: {}\n", e);
      return Ok(());
    }
  };
  let content = &buffer.get_chars().unwrap();
  let content_str = content.as_bytes();

  match file.write(content_str) {
    Err(e) => error!("Problem with writing; {}\n", e),
    Ok(_) => (),
  }

  Ok(())
}
