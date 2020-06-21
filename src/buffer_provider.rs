//Buffer provider
use crate::buffer;

//TODO str is not what we want here for file option, and we need our own errors
pub fn new_buffer(_file_name: Option<&str>) -> Result<impl buffer::Buffer, String> {
    //Err(String::from("not yet implemented"))
    Ok(buffer::LineBuffer::new())
}
