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
      plugins: None,
    }
  }

  // Meant as a place that can locate plugins at a given startup spot,
  // as well as load the core functional plugins. For now, core
  // functional plugins are hard-coded. Dynamic memory appropriate
  // here as we are dealing with variou plugins at runtime here.
  async fn register_core_plugins(&mut self) {
    debug!("Registering core plugins");

    // Diagnostics
    debug!("Registering Diagnostics plugin");
    let diagnostics_plugin : Diagnostics = Diagnostics::new();
    
    self.register_plugin(Box::new(diagnostics_plugin));
    debug!("Diagnostics plugin registered");


    // Buffers
    debug!("Registering Buffers plugin");
    let buffers_plugin : Buffers = Buffers::new();

    self.register_plugin(Box::new(buffers_plugin));
    debug!("Buffers plugin registered");
  }

  pub fn register_plugin(&mut self, mut plugin: Box<dyn Plugin + Send>) {
    plugin.receive_client(self.get_client().unwrap());
    match self.plugins.take() {
      None => self.plugins = Some(vec!(plugin)),
      Some(mut p) => {
	p.push(plugin);
	self.plugins = Some(p);
      },
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
    self.register_core_plugins().await;
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

#[cfg(test)]
mod session_tests {
  use super::Session;
  use crate::plugin::plugin_test_utility::TestPlugin;

  #[test]
  fn session_should_have_no_plugins_when_new() {
    let session = Session::new();
    assert!(session.plugins.is_none())
  }

  #[test]
  fn session_should_add_one_plugin() {
    let mut session = Session::new();
    session.register_plugin(Box::new(TestPlugin::new()));

    assert!(session.plugins.is_some());
    assert_eq!(session.plugins.expect("").len(), 1)
  }

  #[test]
  fn session_should_add_two_plugins() {
    let mut session = Session::new();
    session.register_plugin(Box::new(TestPlugin::new()));
    session.register_plugin(Box::new(TestPlugin::new()));

    assert!(session.plugins.is_some());
    assert_eq!(session.plugins.expect("").len(), 2);
  }
}
