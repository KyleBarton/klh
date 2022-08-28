use log::info;

use crate::plugin::PluginChannel;
use crate::plugin::Plugin;
use crate::plugins::{buffers::Buffers, diagnostics::Diagnostics};
use crate::session::SessionClient;

use super::dispatch::Dispatch;


pub struct Session {
  dispatch: Dispatch,
}


impl Session {
  pub fn new() -> Self {
    Session{
      dispatch: Dispatch::new(),
    }
  }

  // Meant as a place that can locate plugins at a given startup spot,
  // as well as load the core functional plugins. For now, core
  // functional plugins are hard-coded. Dynamic memory appropriate
  // here as we are dealing with variou plugins at runtime here.
  async fn discover_plugins(&mut self) {
    // Diagnostics
    let mut diagnostics_plugin : Diagnostics = Diagnostics::new();
    
    diagnostics_plugin.receive_client(self.get_client().unwrap());

    let mut diagnostics_channel: PluginChannel = PluginChannel::new(Box::new(diagnostics_plugin));

    self.dispatch.register_plugin(&diagnostics_channel).unwrap();


    // Buffers
    let mut buffers_plugin : Buffers = Buffers::new();

    buffers_plugin.receive_client(self.get_client().unwrap());

    let mut buffers_channel: PluginChannel = PluginChannel::new(Box::new(buffers_plugin));

    self.dispatch.register_plugin(&buffers_channel).unwrap();

    tokio::spawn(async move {
      diagnostics_channel.start().await
    });
    tokio::spawn(async move {
      buffers_channel.start().await
    });
  }

  pub async fn run(&mut self) -> Result<(), String> {
    info!("Starting plugins");
    self.discover_plugins().await;
    let readonly_dispatch_options = if self.dispatch.is_uncloned() {
      self.dispatch.clone()
    } else {
      return Err(String::from("Cannot call session run more than once"));
    };

    let mut listener_dispatch = self.dispatch.clone_once();
    
    tokio::spawn( async move {
      listener_dispatch.start_listener().await.unwrap()
    });

    self.dispatch = readonly_dispatch_options;

    Ok(())
  }

  pub fn get_client(&self) -> Result<SessionClient, String> {
    match self.dispatch.get_client() {
      Err(msg) => Err(msg),
      Ok(client) => {
	Ok(SessionClient::new(client))
      }
    }
  }
}
