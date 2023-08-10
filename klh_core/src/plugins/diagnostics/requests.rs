use crate::messaging::{Request, MessageType, MessageContent};

use super::models::SlowBombContent;

pub fn new_log_event() -> Request {
  Request::new(
    MessageType::command_from_str("diagnostics::log_event").unwrap(),
    MessageContent::empty(),
  )
}

pub fn new_slow_bomb(interval_seconds: u64) -> Request {
  Request::new(
    MessageType::command_from_str("diagnostics::slow_bomb").unwrap(),
    MessageContent::from_content(
      SlowBombContent {
	interval_seconds,
      }
    ),
  )
}
