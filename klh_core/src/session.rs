use crate::dispatch::{Dispatch, DispatchClient, DispatchInput, DispatchOptions};

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
#[derive(Copy, Clone, Debug)]
pub struct SessionInput {
  input: DispatchInput,
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
    // Gotta handle the to_owned thing I think
    let readonly_dispatch_options = if self.options.dispatch_options.is_uncloned() {
      self.options.dispatch_options.clone()
    } else {
      return Err(String::from("Cannot call session run more than once"));
    };
    
    Dispatch::start_listener(self.options.dispatch_options.to_owned()).await.unwrap();

    self.options.dispatch_options = readonly_dispatch_options;
    Ok(())
  }

  pub fn get_client(&self) -> Result<SessionClient, String> {
    Ok(SessionClient{
      dispatch_client: Dispatch::get_client(self.options.dispatch_options.clone()).unwrap(),
    })
  }
}
