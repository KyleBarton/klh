use log::debug;

use crate::plugin::PluginChannel;
use crate::plugin::Plugin;
use crate::plugin::PluginRegistrar;
use crate::session::SessionClient;

use super::dispatch::Dispatch;

/// Errors which can occur when using a klh [Session](Session)
#[derive(Debug, Eq, PartialEq)]
pub enum SessionError {
  /// Indicates that a Message was not able to be sent to the central
  /// processing loop of the session.
  ErrorSendingMessage,
  /// Indicates a caller's attempt to start a session using
  /// [Session::run](Session::run) after it has already been
  /// run. [Session](Session) instances can only be run once.
  SessionAlreadyStarted,
}

pub struct Session {
  dispatch: Option<Dispatch>,
  client: SessionClient,
  plugin_registrar: PluginRegistrar,
  plugin_channels: Option<Vec<PluginChannel>>,
}


impl Session {
  pub fn new() -> Self {
    let dispatch = Dispatch::new();
    let client = SessionClient::new(dispatch.get_client());
    Session{
      dispatch: Some(dispatch),
      client,
      plugin_registrar: PluginRegistrar::new(),
      plugin_channels: None,
    }
  }

  /// Registers a plugin with the session. This plugin will be started
  /// on a channel when [Session::run] is called.
  pub fn register_plugin(&mut self, mut plugin: Box<dyn Plugin + Send>) {
    plugin.receive_client(self.get_client());

    let message_types = plugin.list_message_types();

    let channel = PluginChannel::new(plugin);

    self.plugin_registrar.register_message_types_for_plugin(
      message_types,
      channel.get_transmitter(),
    );

    match self.plugin_channels.take() {
      None => self.plugin_channels = Some(vec!(channel)),
      Some(mut channels) => {
	channels.push(channel);
	self.plugin_channels = Some(channels);
      }
    }
  }

  /// Runs the `Session`. This will do the following:
  /// 1. Registers core plugins according to the `KlhConfig`
  /// `CorePlugins` options.
  /// 2. Starts a each registered plugin on a plugin channel
  /// 3. Starts the main processing loop to listen for incoming messages.
  /// # Errors
  /// Returns an [Err] result of [SessionError] in certain situations:
  /// 1. [SessionError::SessionAlreadyStarted] - this session has
  /// already had `run()` called.
  pub async fn run(&mut self) -> Result<(), SessionError> {
    self.start_plugins().await;

    match self.dispatch.take() {
      None => {
	debug!("Attempted to run a used session");
	Err(SessionError::SessionAlreadyStarted)
      },
      Some(mut dispatch) => {
	let registrar_copy = self.plugin_registrar.clone();
	tokio::spawn(async move {
	  dispatch.start(registrar_copy).await.unwrap()
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
  /// let session = Session::new();
  /// let client: SessionClient = session.get_client();
  /// ```
  pub fn get_client(&self) -> SessionClient {
    self.client.clone()
  }

  /// Start registered plugins. Called in the main `run` loop.
  async fn start_plugins(&mut self) {
    debug!("Starting provided plugins");
    match self.plugin_channels.take() {
      None => debug!("No plugins to load"),
      Some(channels) => {
	for mut channel in channels {
	  tokio::spawn(async move {
	    channel.start().await
	  });
	}
      }
    }
  }
}

// impl Default for Session {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[cfg(test)]
mod session_tests {
  use rstest::*;
  use super::Session;
  use crate::messaging::{Request, MessageType, MessageError};
  use crate::session::session::SessionError;
  use crate::plugin::plugin_test_utility::{TestPlugin, QUERY_ID, QUERY_RESPONSE};

  #[fixture]
  fn default_session() -> Session {
    let mut session = Session::new();
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
      MessageType::query_from_str(QUERY_ID).unwrap()
    );
    let mut handler = request.get_handler().unwrap();

    client.send(request.as_message()).await.unwrap();

    let mut response = handler.handle_response().await.unwrap();

    let deserialized_response: String = response.deserialize()
      .expect("Should deserialize into a string");
    assert_eq!(QUERY_RESPONSE.to_string(), deserialized_response);
  }

  #[rstest]
  #[tokio::test]
  async fn should_start_plugin(mut default_session: Session) {
    default_session.start_plugins().await;
  }


  #[rstest]
  #[tokio::test]
  async fn should_handle_unknown_message(mut default_session: Session) {
    default_session.run().await.unwrap();

    let mut client = default_session.get_client();

    let mut unknown_request = Request::from_message_type(
      MessageType::query_from_str("unknown").unwrap()
    );

    let mut handler = unknown_request.get_handler().unwrap();

    client.send(unknown_request.as_message()).await.unwrap();

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
    let session = Session::new();
    assert!(session.plugin_channels.is_none())
  }

  #[rstest]
  fn session_should_add_one_plugin() {
    let mut session = Session::new();
    session.register_plugin(Box::new(TestPlugin::new()));

    assert!(session.plugin_channels.is_some());
    assert_eq!(session.plugin_channels.expect("").len(), 1)
  }

  #[rstest]
  fn session_should_add_two_plugins() {
    let mut session = Session::new();
    session.register_plugin(Box::new(TestPlugin::new()));
    session.register_plugin(Box::new(TestPlugin::new()));

    assert!(session.plugin_channels.is_some());
    assert_eq!(session.plugin_channels.expect("").len(), 2);
  }
}
