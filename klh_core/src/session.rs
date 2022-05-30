use crate::dispatch::{Dispatcher, DispatchClient, DispatchOptions};
use crate::event::Event;

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

pub struct Session {
  options: SessionOptions, // TODO lifetime?
}

// TODO impl client commands
pub struct SessionClient{
  dispatch_client: DispatchClient,
}

// Needs so much work
impl SessionClient {
  pub async fn send(&mut self, event: Event) {
    self.dispatch_client.send(event).await.unwrap()
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
