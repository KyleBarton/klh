use crate::dispatch::{Dispatcher, DispatchClient, Dispatch};
use crate::event::EventMessage;
use crate::plugin::{Plugin, PluginChannel};
use crate::plugins::buffers::Buffers;
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

pub struct SessionClient{
  dispatch_client: DispatchClient,
}

// Needs so much work
impl SessionClient {

  pub async fn send(&mut self, event_message: EventMessage) -> Result<(), String> {
    match self.dispatch_client.send(event_message).await {
      Err(_) => Err("Issue sending event message to session".to_string()),
      Ok(_) => Ok(())
    }
  }
}

pub struct Session {
  options: SessionOptions,
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
  pub(crate) async fn discover_plugins(&mut self) {
    // Diagnostics
    let mut diagnostics_plugin : Diagnostics = Diagnostics::new();
    
    diagnostics_plugin.receive_client(self.get_client().unwrap());

    let mut diagnostics_channel: PluginChannel = PluginChannel::new(Box::new(diagnostics_plugin));

    self.options.dispatch.register_plugin(diagnostics_channel.get_transmitter().unwrap()).unwrap();


    // Buffers
    let mut buffers_plugin : Buffers = Buffers::new();

    buffers_plugin.receive_client(self.get_client().unwrap());

    let mut buffers_channel: PluginChannel = PluginChannel::new(Box::new(buffers_plugin));

    self.options.dispatch.register_plugin(buffers_channel.get_transmitter().unwrap()).unwrap();

    tokio::spawn(async move {
      diagnostics_channel.start().await
    });
    tokio::spawn(async move {
      buffers_channel.start().await
    });
  }

  pub async fn run(&mut self) -> Result<(), String> {
    println!("Starting plugins");
    self.discover_plugins().await;
    let readonly_dispatch_options = if self.options.dispatch.is_uncloned() {
      self.options.dispatch.clone()
    } else {
      return Err(String::from("Cannot call session run more than once"));
    };

    let listener_dispatch = self.options.dispatch.clone_once();
    
    tokio::spawn( async move {
      Dispatcher::start_listener(listener_dispatch).await.unwrap()
    });


    self.options.dispatch = readonly_dispatch_options;

    Ok(())
  }

  pub fn get_client(&self) -> Result<SessionClient, String> {
    Ok(SessionClient{
      dispatch_client: self.options.dispatch.get_client().unwrap(),
    })
  }
}
