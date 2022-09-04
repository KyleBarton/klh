use log::debug;

use crate::plugin::PluginChannel;
use crate::plugin::Plugin;
use crate::plugins::{buffers::Buffers, diagnostics::Diagnostics};
use crate::session::SessionClient;

use super::dispatch::Dispatch;


pub struct Session {
  dispatch: Dispatch,
  plugins: Option<Vec<Box<dyn Plugin + Send>>>,
}


impl Session {
  pub fn new() -> Self {
    Session{
      dispatch: Dispatch::new(),
      plugins: Some(Vec::new()),
    }
  }

  // Meant as a place that can locate plugins at a given startup spot,
  // as well as load the core functional plugins. For now, core
  // functional plugins are hard-coded. Dynamic memory appropriate
  // here as we are dealing with variou plugins at runtime here.
  async fn start_core_plugins(&mut self) {
    debug!("Loading core plugins");
    // Diagnostics
    debug!("Started loading Diagnostics plugin");
    let mut diagnostics_plugin : Diagnostics = Diagnostics::new();
    
    diagnostics_plugin.receive_client(self.get_client().unwrap());

    let mut diagnostics_channel: PluginChannel = PluginChannel::new(Box::new(diagnostics_plugin));

    self.dispatch.register_plugin(&diagnostics_channel).unwrap();
    debug!("Diagnostics plugin loaded");


    // Buffers
    debug!("Started loading Buffers plugin");
    let mut buffers_plugin : Buffers = Buffers::new();

    buffers_plugin.receive_client(self.get_client().unwrap());

    let mut buffers_channel: PluginChannel = PluginChannel::new(Box::new(buffers_plugin));

    self.dispatch.register_plugin(&buffers_channel).unwrap();
    debug!("Buffers plugin loaded");

    tokio::spawn(async move {
      diagnostics_channel.start().await
    });
    debug!("Diagnostics plugin listening");
    tokio::spawn(async move {
      buffers_channel.start().await
    });
    debug!("Buffers plugin listening");
  }


  pub fn register_plugins(&mut self, mut plugins: Vec<Box<dyn Plugin + Send>>) {
    match self.plugins.take() {
      None => self.plugins = Some(plugins),
      Some(mut p) => {
	p.append(&mut plugins);
	self.plugins = Some(p);
      }
    }
  }

  async fn start_plugins(&mut self) {
    debug!("Starting provided plugins");
    match self.plugins.take() {
      None => panic!("no plugins"),
      Some(plugins) => {
	for plugin in plugins {
	  let mut plugin_channel: PluginChannel = PluginChannel::new(plugin);
	  self.dispatch.register_plugin(&plugin_channel).unwrap();
	  tokio::spawn(async move {
	    plugin_channel.start().await
	  });
	}
      }
    }
  }

  pub async fn run(&mut self) -> Result<(), String> {
    self.start_core_plugins().await;
    self.start_plugins().await;

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
