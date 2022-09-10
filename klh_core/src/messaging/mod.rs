mod message_content;
pub use message_content::*;

mod response;
pub use response::*;

mod request;
pub use request::*;

mod message_type;
pub use message_type::*;

mod message;
pub use message::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum MessageError {
  MessageTypeNotFound,
}
