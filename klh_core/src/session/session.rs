use log::debug;
use log::error;

use crate::config::CorePlugins;
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
  config: KlhConfig,
  dispatch: Option<Dispatch>,
  client: SessionClient,
  plugins: Option<Vec<Box<dyn Plugin + Send>>>,
}


impl Session {
  pub fn new(config: KlhConfig) -> Self {
    let dispatch = Dispatch::new();
    let client = SessionClient::new(dispatch.get_client().unwrap());
    Session{
      config,
      dispatch: Some(dispatch),
      client,
      plugins: None,
    }
  }

  // Meant as a place that can locate plugins at a given startup spot,
  // as well as load the core functional plugins. For now, core
  // functional plugins are hard-coded. Dynamic memory appropriate
  // here as we are dealing with variou plugins at runtime here.
  fn register_core_plugins(&mut self) {
    debug!("Registering core plugins");

    let mut core_plugins : Vec<Box<dyn Plugin + Send>> = Vec::new();
    for core_plugin in &self.config.core_plugins {
      match core_plugin {
	CorePlugins::Diagnostics => {
	  debug!("Adding Diagnostics plugin");
	  core_plugins.push(Box::new(Diagnostics::new()));
	  debug!("Diagnostics plugin added");
	},
	CorePlugins::Buffers => {
	  debug!("Adding Buffers plugin");
	  core_plugins.push(Box::new(Buffers::new()));
	  debug!("Buffers plugin added");
	},
	_ => (),
      }
    }

    for plugin in core_plugins {
      self.register_plugin(plugin);
    }
  }

  /// Registers a plugin with the session. This plugin will be started
  /// on a channel when [Session::run] is called.
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
	  }
	}
	self.dispatch = Some(dispatch);
      }
    }
    
  }

  /// Runs the `Session`. This will do the following:
  /// 1. Registers core plugins according to the `KlhConfig`
  /// `CorePlugins` options.
  /// 2. Starts a each registered plugin on a plugin channel
  /// 3. Starts the `Dispatch` to listen for incoming messages.
  /// # Errors
  /// Returns an [Err] result of [SessionError] in certain situations:
  /// 1. [SessionError::SessionAlreadyStarted] - this session has
  /// already had `run()` called.
  pub async fn run(&mut self) -> Result<(), SessionError> {
    self.register_core_plugins();
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

  /// Returns a new `SessionClient`
  /// # Examples
  /// ```
  /// use klh_core::config::KlhConfig;
  /// use klh_core::session::*;
  /// 
  /// let session = Session::new(KlhConfig::default());
  /// let client: SessionClient = session.get_client();
  /// ```
  pub fn get_client(&self) -> SessionClient {
    self.client.clone()
  }
}

#[cfg(test)]
mod session_tests {
  use rstest::*;
  use super::Session;
  use crate::{plugin::plugin_test_utility::{TestPlugin, QUERY_ID, QUERY_RESPONSE}, messaging::{Request, MessageType, MessageError}, session::session::SessionError, config::{KlhConfig, CorePlugins}, plugins::buffers::{requests::new_list_buffers_request, models::ListBuffersResponse}};

  #[fixture]
  fn default_session() -> Session {
    let mut session = Session::new(KlhConfig::default());
    let plugin: TestPlugin = TestPlugin::new();
    session.register_plugin(Box::new(plugin));
    session
  }

  #[rstest]
  #[tokio::test]
  async fn should_send_message_to_registered_plugin(mut default_session: Session) {
    let mut client = default_session.get_client();
    default_session.run().await.unwrap();
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
  async fn should_start_plugin(mut default_session: Session) {
    default_session.start_plugins().await;
  }


  #[rstest]
  #[tokio::test]
  async fn should_register_core_plugin_specified_in_config() {
    let config = KlhConfig::with_core_plugins(vec!(CorePlugins::Buffers));

    let mut session = Session::new(config);

    session.run().await.unwrap();

    let mut request = new_list_buffers_request();

    let mut handler = request.get_handler().unwrap();

    session.get_client().send(request.to_message().unwrap()).await.unwrap();

    let mut response = handler.handle_response().await.unwrap();

    let expected_response : Option<ListBuffersResponse> = response.deserialize();

    assert!(expected_response.is_some());

  }

  #[rstest]
  #[tokio::test]
  async fn should_not_register_core_plugins_omitted_in_config() {
    let config = KlhConfig::with_core_plugins(Vec::new());

    let mut session = Session::new(config);

    session.run().await.unwrap();

    let mut request = new_list_buffers_request();

    let mut handler = request.get_handler().unwrap();

    session.get_client().send(request.to_message().unwrap()).await.unwrap();

    let mut response = handler.handle_response().await.unwrap();

    let mut response_as_error = response.clone();

    let unexpected_response : Option<ListBuffersResponse> = response.deserialize();

    assert!(unexpected_response.is_none());

    let expected_error_response : Option<MessageError> = response_as_error.deserialize();

    assert_eq!(expected_error_response.expect("should have error"), MessageError::MessageTypeNotFound)
  }

  #[rstest]
  #[tokio::test]
  async fn should_handle_unknown_message(mut default_session: Session) {
    default_session.run().await.unwrap();

    let mut client = default_session.get_client();

    let mut unknown_request = Request::from_message_type(MessageType::query_from_str("unknown"));

    let mut handler = unknown_request.get_handler().unwrap();

    client.send(unknown_request.to_message().unwrap()).await.unwrap();

    let mut response = handler.handle_response().await.unwrap();

    let expected_response: Option<MessageError> = response.deserialize();

    assert_eq!(expected_response, Some(MessageError::MessageTypeNotFound));
  }

  #[rstest]
  #[tokio::test]
  async fn should_not_be_able_to_start_started_session(mut default_session: Session) {
    default_session.run().await.unwrap();

    let unsuccessful_run = default_session.run().await;

    assert!(unsuccessful_run.is_err());
    assert_eq!(unsuccessful_run.err(), Some(SessionError::SessionAlreadyStarted));
  }

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
