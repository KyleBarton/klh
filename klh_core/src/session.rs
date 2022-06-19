use crate::dispatch::{Dispatcher, DispatchClient, Dispatch};
use crate::event::Event;
use crate::plugin::{Plugin, PluginChannel};
use crate::plugins::diagnostics::Diagnostics;

#[derive(Clone)]
pub struct SessionOptions {
  dispatch: Dispatch,
}

impl SessionOptions {
  pub fn new() -> Self {
    Self {
      dispatch: Dispatch::new(),
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

    let mut diagnostics_plugin : Diagnostics = Diagnostics::new();
    
    diagnostics_plugin.receive_client(self.options.dispatch.get_client().unwrap());

    let plugin_channel: PluginChannel = PluginChannel::new(Box::new(diagnostics_plugin));

    self.options.dispatch.register_plugin(plugin_channel.get_transmitter().unwrap()).unwrap();

    println!("Diagnostics plugin registered");
  }

  // TODO Clean up result signature
  // Starts the async runtime
  pub async fn run(&mut self) -> Result<(), String> {
    let readonly_dispatch_options = if self.options.dispatch.is_uncloned() {
      self.options.dispatch.clone()
    } else {
      return Err(String::from("Cannot call session run more than once"));
    };

    
    Dispatcher::start_listener(self.options.dispatch.clone_once()).await.unwrap();

    self.options.dispatch = readonly_dispatch_options;

    Ok(())
  }

  pub fn get_client(&self) -> Result<SessionClient, String> {
    Ok(SessionClient{
      dispatch_client: Dispatcher::get_client(self.options.dispatch.clone()).unwrap(),
    })
  }
}
