use crate::buffer;
use crate::buffer::buffer_provider;

pub struct BufferStoreArgs {
    buffer_type: buffer_provider::BufferType,
    file_name: String,
}

impl BufferStoreArgs {
  pub fn new(buffer_type: buffer_provider::BufferType, file_name: &str) -> BufferStoreArgs {
    BufferStoreArgs {
      buffer_type,
      file_name: String::from(file_name),
    }
  }
}

pub struct BufferStore {
    buffers: Vec<Box<dyn buffer::Buffer>>,
}

// TODO we'll figure this out when the time comes
// impl Iterator for BufferStore {
//     type Item = Box<dyn buffer::Buffer>;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

//TODO allow more than one buffer
impl BufferStore {
    pub fn new() -> Self {
        BufferStore {
            buffers: vec![],
        }
    }
  pub fn add_new(&mut self, args: BufferStoreArgs) -> Option<&Box<dyn buffer::Buffer>> {
    let buffer = match args.file_name.as_str() {
      "" => buffer_provider::new(args.buffer_type).unwrap(),
      _ => buffer_provider::from_file(args.buffer_type, &args.file_name).unwrap(),
    };

    self.buffers.push(buffer);

    self.buffers.get(0)
    
  }

  //TODO protect against empty I guess? Maybe option does that for us
  pub fn get_current_mut(&mut self) -> Result<&mut Box<dyn buffer::Buffer>, &str> {
    match self.buffers.get_mut(0) {
      None => Err("No"),
      Some(b) => Ok(b),
    }
  }
}
