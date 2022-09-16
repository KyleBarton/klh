use log::debug;

use crate::{messaging::Request, session::{Session, SessionClient}, plugin::Plugin, config::KlhConfig};

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

  pub async fn send(&mut self, mut request: Request) -> Result<(), String> {
    match self.session_client.send(
      request.to_message()
    ).await {
      Err(session_err) => {
	debug!("Error sending message to session: {:?}", session_err);
	Err("Error sending message to session".to_string())
      },
      Ok(_) => Ok(()),
    }
  }
}

pub struct Klh {
  session: Session,
}

impl Klh {
  pub fn new() -> Self {
    let session = Session::new(KlhConfig::default());
    Self {
      session,
    }
  }

  pub async fn start(&mut self) {
    self.session.run().await.unwrap();
    debug!("Session started successfully");
  }

  pub fn add_plugin(&mut self, plugin: Box<dyn Plugin + Send>) {
    self.session.register_plugin(plugin)
  }

  pub fn get_client(&self) -> KlhClient {
    // TODO eliminate this result chain
    let client = KlhClient::new(self.session.get_client());
    client
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
