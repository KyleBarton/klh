use log::debug;
use log::error;

use crate::plugin::PluginChannel;
use crate::plugin::Plugin;
use crate::plugins::{buffers::Buffers, diagnostics::Diagnostics};
use crate::session::SessionClient;

use super::dispatch::Dispatch;


pub struct Session {
  dispatch: Option<Dispatch>,
  client: SessionClient,
  plugins: Option<Vec<Box<dyn Plugin + Send>>>,
}


impl Session {
  pub fn new() -> Self {
    let dispatch = Dispatch::new();
    let client = SessionClient::new(dispatch.get_client().unwrap());
    Session{
      dispatch: Some(dispatch),
      client,
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
    plugin.receive_client(self.get_client());
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
    match self.dispatch.take() {
      None => error!("Tried to start plugins after session started"),
      Some(mut dispatch) => {
	match self.plugins.take() {
	  None => debug!("No plugins to load"),
	  Some(plugins) => {
	    for plugin in plugins {
	      let mut plugin_channel: PluginChannel = PluginChannel::new(plugin);
	      dispatch.register_plugin(&plugin_channel).unwrap();
	      tokio::spawn(async move {
		plugin_channel.start().await
	      });
	    }
	    self.dispatch = Some(dispatch);
	  }
	}
      }
    }
    
  }
  pub async fn run(&mut self) -> Result<(), String> {
    self.register_core_plugins().await;
    self.start_plugins().await;

    match self.dispatch.take() {
      None => Err("Session already used".to_string()),
      Some(mut dispatch) => {
	tokio::spawn(async move {
	  dispatch.start_listener().await.unwrap()
	});
	Ok(())
      }
    }
  }

  pub fn get_client(&self) -> SessionClient {
    self.client.clone()
  }
}

#[cfg(test)]
mod session_tests {
  use rstest::*;
  use super::Session;
  use crate::{plugin::plugin_test_utility::{TestPlugin, QUERY_ID, QUERY_RESPONSE}, messaging::{Request, MessageType}};

  #[fixture]
  pub fn session_with_plugin_fixture() -> Session {
    let mut session = Session::new();
    let plugin: TestPlugin = TestPlugin::new();
    session.register_plugin(Box::new(plugin));
    session
  }

  #[fixture]
  pub fn session_with_no_plugins_fixture() -> Session {
    Session::new()
  }

  #[rstest]
  #[tokio::test]
  pub async fn should_send_message_to_registered_plugin(mut session_with_plugin_fixture: Session) {
    let mut client = session_with_plugin_fixture.get_client();
    session_with_plugin_fixture.run().await.unwrap();
    let mut request = Request::from_message_type(
      MessageType::query_from_str(QUERY_ID)
    );
    let mut handler = request.get_handler().unwrap();

    client.send(request.to_message().unwrap()).await.unwrap();

    let mut response = handler.handle_response().await.unwrap();

    let deserialized_response: String = response.deserialize()
      .expect("Should deserialize into a string");
    assert_eq!(QUERY_RESPONSE.to_string(), deserialized_response);
  }

  #[rstest]
  pub fn should_register_plugin() { 
    let mut session = Session::new();

    session.register_plugin(Box::new(TestPlugin::new()));

    assert!(session.plugins.is_some());
    assert_eq!(1, session.plugins.expect("Should have a plugin").len());
    
  }

  // TODO Not sure if I really need this
  // How _do_ I test this part of it?
  #[rstest]
  #[tokio::test]
  pub async fn should_start_plugin(mut session_with_plugin_fixture: Session) {
    session_with_plugin_fixture.start_plugins().await;
  }


  // test - should register correct core plugins according to config (many tests & design needed)

  // test - client should send unknown message and its handled the right way
  // TODO This doesn't work, because the message is fire & forget. Think about hwo best to test this.
  // #[rstest]
  // #[tokio::test]
  // pub async fn should_handle_unknown_message(mut session_with_plugin_fixture: Session) {
  //   session_with_plugin_fixture.run().await.unwrap();

  //   let mut client = session_with_plugin_fixture.get_client();

  //   let mut unknown_request = Request::from_message_type(MessageType::query_from_str("unknown"));

  //   let result = client.send(unknown_request.to_message().unwrap()).await;
  //   assert!(result.is_err());
  //   assert_eq!(result.err().expect("Should error"), "Unrecognized Message Type".to_string());
  // }

  // test - should start session

  // test - should not be able to start already started session

  /// Actually maybe we support the below
  // test - should not be able to register plugins after session started

  // test - should not be able to start plugins after session started
  // IDK Maybe this is actually something we support

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
