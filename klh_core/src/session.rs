use std::thread;

use tokio::runtime::{Runtime, self};

use crate::dispatch::{Dispatcher, DispatchClient, Dispatch};
use crate::event::Event;
use crate::plugin::{Plugin, PluginChannel, PluginStarter};
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
  pub async fn send(&mut self, event: Event) -> Result<(), String> {
    match self.dispatch_client.send(event).await {
      Err(_) => Err("issue sending event to session".to_string()),
      Ok(_) => Ok(())
    }
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
  pub(crate) async fn discover_plugins(&mut self) {
    let mut diagnostics_plugin : Diagnostics = Diagnostics::new();
    
    diagnostics_plugin.receive_client(self.options.dispatch.get_client().unwrap());

    let mut plugin_channel: PluginChannel = PluginChannel::new(Box::new(diagnostics_plugin));

    self.options.dispatch.register_plugin(plugin_channel.get_transmitter().unwrap()).unwrap();


    tokio::spawn(async move {
      plugin_channel.start().await
    });

    // TODO need to figure out the borrowing for a different function to run the plugins
    // self.channels.push(plugin_channel);

    // Start the plugin channel on another thread.
    // TODO join?

    // let plugin_listener = plugin_channel.listener;
    
    // let plugin_thread = thread::spawn(move || {
    //   let runtime = Runtime::new().unwrap();
    //   runtime.spawn(async move {
    // 	println!("Diagnostics plugin registered, and started");
    // 	PluginStarter::start_plugin(plugin_channel).await.unwrap();
    //   })
    // });

  }

  // TODO Clean up result signature
  // Starts the async runtime
  pub async fn run(&mut self) -> Result<(), String> {
    println!("Starting plugins");
    // TODO probably don't need to make these async, since the thread runtimes handle it
    self.discover_plugins().await;
    // self.discover_plugins().await;
    let readonly_dispatch_options = if self.options.dispatch.is_uncloned() {
      self.options.dispatch.clone()
    } else {
      return Err(String::from("Cannot call session run more than once"));
    };

    let listener_dispatch = self.options.dispatch.clone_once();
    
    // TODO join?
    // let session_thread = thread::spawn(move || {
    //   let runtime = Runtime::new().unwrap();
    //   runtime.spawn(async move {
    // 	println!("Dispatcher awaiting commands to the session");
    // 	Dispatcher::start_listener(listener_dispatch).await.unwrap();
    //   })
    // });
    tokio::spawn( async move {
      Dispatcher::start_listener(listener_dispatch).await.unwrap()
    });


    self.options.dispatch = readonly_dispatch_options;

    // TODO probably needs to be done in an "end" function
    // session_thread.join().unwrap();

    Ok(())
  }

  pub fn get_client(&self) -> Result<SessionClient, String> {
    Ok(SessionClient{
      dispatch_client: Dispatcher::get_client(self.options.dispatch.clone()).unwrap(),
    })
  }
}
