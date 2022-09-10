use log::debug;
use log::error;

use crate::config::KlhConfig;
use crate::plugin::PluginChannel;
use crate::plugin::Plugin;
use crate::plugins::{buffers::Buffers, diagnostics::Diagnostics};
use crate::session::SessionClient;

use super::dispatch::Dispatch;

#[derive(Debug, Eq, PartialEq)]
pub enum SessionError {
  SessionAlreadyStarted,
}

pub struct Session {
  _config: KlhConfig,
  dispatch: Option<Dispatch>,
  client: SessionClient,
  plugins: Option<Vec<Box<dyn Plugin + Send>>>,
}


impl Session {
  pub fn new(config: KlhConfig) -> Self {
    let dispatch = Dispatch::new();
    let client = SessionClient::new(dispatch.get_client().unwrap());
    Session{
      _config: config,
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
  pub async fn run(&mut self) -> Result<(), SessionError> {
    self.register_core_plugins().await;
    self.start_plugins().await;

    match self.dispatch.take() {
      None => {
	debug!("Attempted to run a used session");
	Err(SessionError::SessionAlreadyStarted)
      },
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
  use crate::{plugin::plugin_test_utility::{TestPlugin, QUERY_ID, QUERY_RESPONSE}, messaging::{Request, MessageType, MessageError}, session::session::SessionError, config::KlhConfig};

  #[fixture]
  fn session_with_plugin_fixture() -> Session {
    let mut session = Session::new(KlhConfig::default());
    let plugin: TestPlugin = TestPlugin::new();
    session.register_plugin(Box::new(plugin));
    session
  }

  // #[fixture]
  // fn session_with_no_plugins_fixture() -> Session {
  //   Session::new(KlhConfig::default())
  // }

  #[rstest]
  #[tokio::test]
  async fn should_send_message_to_registered_plugin(mut session_with_plugin_fixture: Session) {
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
  fn should_register_plugin() { 
    let mut session = Session::new(KlhConfig::default());

    session.register_plugin(Box::new(TestPlugin::new()));

    assert!(session.plugins.is_some());
    assert_eq!(1, session.plugins.expect("Should have a plugin").len());
    
  }

  #[rstest]
  #[tokio::test]
  async fn should_start_plugin(mut session_with_plugin_fixture: Session) {
    session_with_plugin_fixture.start_plugins().await;
  }


  // test - should register correct core plugins according to config (many tests & design needed)

  #[rstest]
  #[tokio::test]
  async fn should_handle_unknown_message(mut session_with_plugin_fixture: Session) {
    session_with_plugin_fixture.run().await.unwrap();

    let mut client = session_with_plugin_fixture.get_client();

    let mut unknown_request = Request::from_message_type(MessageType::query_from_str("unknown"));

    let mut handler = unknown_request.get_handler().unwrap();

    client.send(unknown_request.to_message().unwrap()).await.unwrap();

    let mut response = handler.handle_response().await.unwrap();

    let expected_response: Option<MessageError> = response.deserialize();

    assert_eq!(expected_response, Some(MessageError::MessageTypeNotFound));
  }

  #[rstest]
  #[tokio::test]
  async fn should_not_be_able_to_start_started_session(mut session_with_plugin_fixture: Session) {
    session_with_plugin_fixture.run().await.unwrap();

    let unsuccessful_run = session_with_plugin_fixture.run().await;

    assert!(unsuccessful_run.is_err());
    assert_eq!(unsuccessful_run.err(), Some(SessionError::SessionAlreadyStarted));
  }

  // TODO is this really right? Seems like maybe we want plugins to
  // include core plugins now that we have a config.
  #[rstest]
  fn session_should_have_no_plugins_when_new() {
    let session = Session::new(KlhConfig::default());
    assert!(session.plugins.is_none())
  }

  #[rstest]
  fn session_should_add_one_plugin() {
    let mut session = Session::new(KlhConfig::default());
    session.register_plugin(Box::new(TestPlugin::new()));

    assert!(session.plugins.is_some());
    assert_eq!(session.plugins.expect("").len(), 1)
  }

  #[rstest]
  fn session_should_add_two_plugins() {
    let mut session = Session::new(KlhConfig::default());
    session.register_plugin(Box::new(TestPlugin::new()));
    session.register_plugin(Box::new(TestPlugin::new()));

    assert!(session.plugins.is_some());
    assert_eq!(session.plugins.expect("").len(), 2);
  }
}
