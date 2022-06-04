use crate::dispatch::{Dispatcher, DispatchClient, Dispatch};
use crate::event::Event;
use crate::plugin::Plugin;
use crate::plugins::diagnostics::Diagnostics;

#[derive(Clone)]
pub struct SessionOptions {
  dispatch_options: Dispatch,
}

impl SessionOptions {
  pub fn new() -> Self {
    Self {
      dispatch_options: Dispatch::new(),
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

  // Meant as a place that can locate plugins at a given startup spot,
  // as well as load the core functional plugins. For now, core
  // functional plugins are hard-coded. Dynamic memory appropriate
  // here as we are dealing with variou plugins at runtime here.
  fn discover_plugins(&mut self) {
    // Ugh I need to just create a plugin
    // let plugins : [impl Plugin] = [];

    let diagnostics_plugin : Diagnostics = Diagnostics::new();
    
    diagnostics_plugin.receive_client(self.options.dispatch_options.get_client().unwrap());

    self.options.dispatch_options.register_plugin(diagnostics_plugin).unwrap();
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
