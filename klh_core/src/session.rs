use std::str;

use crate::dispatch::{Dispatcher, DispatchClient, DispatchInput, DispatchOptions};

#[derive(Clone)]
pub struct SessionOptions {
  dispatch_options: DispatchOptions,
}

impl SessionOptions {
  pub fn new() -> Self {
    Self {
      dispatch_options: DispatchOptions::new(),
    }
  }
}

// TODO Placeholder we need to figure out the whole command structure
#[derive(Clone, Debug)]
pub struct SessionInput {
  input: DispatchInput,
}

impl SessionInput {
  pub fn from(msg: &str) -> Self {
    Self {
      input: DispatchInput::Test(String::from(msg)),
    }
  }
}

pub struct Session {
  options: SessionOptions, // TODO lifetime?
}

// TODO impl client commands
pub struct SessionClient{
  dispatch_client: DispatchClient,
}

// Needs so much work
impl SessionClient {
  pub async fn send(&mut self, input: SessionInput) {
    self.dispatch_client.send(input.input).await.unwrap()
  }
}

impl Session {
  // Generate async channels, load plugins. Do not yet start runtime.
  pub fn new(options: SessionOptions) -> Self {
    Session{
      options,
    }
  }

  // TODO Clean up result signature
  // Starts the async runtime
  pub async fn run(&mut self) -> Result<(), String> {
    let readonly_dispatch_options = if self.options.dispatch_options.is_uncloned() {
      self.options.dispatch_options.clone()
    } else {
      return Err(String::from("Cannot call session run more than once"));
    };

    
    Dispatcher::start_listener(self.options.dispatch_options.clone_once()).await.unwrap();

    self.options.dispatch_options = readonly_dispatch_options;

    Ok(())
  }

  pub fn get_client(&self) -> Result<SessionClient, String> {
    Ok(SessionClient{
      dispatch_client: Dispatcher::get_client(self.options.dispatch_options.clone()).unwrap(),
    })
  }
}
