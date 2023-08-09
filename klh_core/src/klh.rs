use log::debug;

use crate::config::{KlhConfig, CorePlugins};
use crate::messaging::Request;
use crate::plugin::Plugin;
use crate::plugins::{diagnostics::Diagnostics, buffers::Buffers};
use crate::session::{Session, SessionClient, SessionError};


#[derive(Debug, Eq, PartialEq)]
pub enum KlhError {
  /// Indicates that a Message was not able to be sent to the running
  /// Klh instance. Wraps a
  /// [SessionError](crate::session::SessionError)
  ErrorSendingMessage(SessionError),
}

/// The entrypoint to sending data through KLH using a [Request](crate::messaging::Request)
#[derive(Clone)]
pub struct KlhClient {
  session_client: SessionClient,
}

impl KlhClient {
  pub fn new(session_client: SessionClient) -> Self {
    Self {
      session_client,
    }
  }

  /// Aynchronously send a [Request](crate::messaging::Request) along
  /// to the running instance of KLH.
  pub async fn send(&mut self, mut request: Request) -> Result<(), KlhError> {
    match self.session_client.send(
      request.to_message()
    ).await {
      Err(session_err) => {
	debug!("Error sending message to session: {:?}", session_err);
	Err(KlhError::ErrorSendingMessage(session_err))
      },
      Ok(_) => Ok(()),
    }
  }
}

/// The primary struct of a running KLH instance.
pub struct Klh {
  session: Session,
  config: KlhConfig,
}

impl Klh {
  pub fn new() -> Self {
    let session = Session::new();
    Self {
      session,
      config: KlhConfig::default(),
    }
  }

  /// Start the instance of Klh, after registering the core plugins
  /// according to its [KlhConfig](crate::config::KlhConfig)
  pub async fn start(&mut self) {
    self.register_core_plugins();
    self.session.run().await.unwrap();
    debug!("Session started successfully");
  }

  /// Add an instance of a [Plugin](crate::plugin::Plugin) to the
  /// session for registration. Must be called before [Klh::start]
  pub fn add_plugin(&mut self, plugin: Box<dyn Plugin + Send>) {
    self.session.register_plugin(plugin)
  }

  /// Provide an instance of a [KlhClient] in order to send it
  /// messages.
  pub fn get_client(&self) -> KlhClient {
    let client = KlhClient::new(self.session.get_client());
    client
  }

  /// Meant as a place that can locate plugins at a given startup spot,
  /// as well as load the core functional plugins. For now, core
  /// functional plugins are hard-coded. Dynamic memory appropriate
  /// here as we are dealing with variou plugins at runtime here.
  fn register_core_plugins(&mut self) {
    for core_plugin in &self.config.core_plugins.clone() {
      match core_plugin {
	CorePlugins::Diagnostics => {
	  debug!("Adding Diagnostics plugin");
	  self.add_plugin(Box::new(Diagnostics::new()));
	  debug!("Diagnostics plugin added");
	},
	CorePlugins::Buffers => {
	  debug!("Adding Diagnostics plugin");
	  self.add_plugin(Box::new(Buffers::new()));
	  debug!("Diagnostics plugin added");
	},
	_ => (),
      }
    }
  }
}


#[cfg(test)]
mod end_to_end_tests {

  use log::debug;
use rstest::{fixture, rstest};

use crate::klh::Klh;
  use crate::messaging::{Request, MessageType};
  use crate::plugin::plugin_test_utility::{TestPlugin, COMMAND_ID, COMMAND_RESPONSE, QUERY_ID, QUERY_RESPONSE};

  // Option thing to set up if you need to debug
  #[fixture]
  #[once]
  fn setup_logging_fixture() -> () {
    simplelog::TermLogger::init(
      simplelog::LevelFilter::Debug,
      simplelog::Config::default(),
      simplelog::TerminalMode::Stdout,
      simplelog::ColorChoice::Auto,
    ).unwrap()
  }

  #[rstest]
  #[ignore]
  fn setup(_setup_logging_fixture: &()) {
    debug!("Setup function completed")
  }

  #[tokio::test]
  async fn should_send_command_request_and_get_response() {
    let mut klh = Klh::new();

    let test_plugin = TestPlugin::new();

    klh.add_plugin(Box::new(test_plugin));

    let mut klh_client = klh.get_client();
    tokio::spawn(async move {
      klh.start().await;
    }).await.unwrap();

    let mut request = Request::from_message_type(MessageType::command_from_str(COMMAND_ID));
    let mut handler = request.get_handler().unwrap();

    klh_client.send(request).await.unwrap();

    let mut response = handler.handle_response().await.unwrap();

    let response_deserialized: String = response.deserialize().expect("Serialize correctly");
    assert_eq!(COMMAND_RESPONSE.to_string(), response_deserialized);
  }

  #[tokio::test]
  async fn should_send_query_request_and_get_response() {
    let mut klh = Klh::new();
    
    let test_plugin = TestPlugin::new();

    klh.add_plugin(Box::new(test_plugin));

    let mut klh_client = klh.get_client();
    tokio::spawn(async move {
      klh.start().await;
    }).await.unwrap();

    let mut request = Request::from_message_type(MessageType::query_from_str(QUERY_ID));
    let mut handler = request.get_handler().unwrap();

    klh_client.send(request).await.unwrap();

    let mut response = handler.handle_response().await.unwrap();

    let response_deserialized: String = response.deserialize().expect("Serialize correctly");
    assert_eq!(QUERY_RESPONSE.to_string(), response_deserialized);
  }

  
}
