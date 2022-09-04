use klh_core::klh::Klh;
use klh_core::plugin::Plugin;
use klh_core::messaging::{Message, MessageType, Request, MessageContent};
use klh_core::session::SessionClient;

const QUERY_ID: &str = "queryId";
const COMMAND_ID: &str = "commandId";
const COMMAND_RESPONSE: &str = "commandResponse";
const QUERY_RESPONSE: &str = "queryResponse";

fn setup_logging() {
  simplelog::TermLogger::init(
    simplelog::LevelFilter::Debug,
    simplelog::Config::default(),
    simplelog::TerminalMode::Stdout,
    simplelog::ColorChoice::Auto,
  ).unwrap();
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

struct TestPlugin {
  command_sent: bool,
  query_sent: bool,
}

impl TestPlugin {
  pub fn new() -> Self {
    Self {
      command_sent: false,
      query_sent: false,
    }
  }
}

impl Plugin for TestPlugin {
  fn accept_message(&mut self, mut message: Message) -> Result<(), String> {
    match message.get_message_type() {
      MessageType::Query(id) => {
	if QUERY_ID.as_bytes() == &id[0..QUERY_ID.len()] {
	  self.query_sent = true;
	  message.get_responder()
	    .expect("Should have responder")
	    .respond(MessageContent::from_content(QUERY_RESPONSE.to_string()))
	    .unwrap();
	}
      },
      MessageType::Command(id) => {
	if COMMAND_ID.as_bytes() == &id[0..COMMAND_ID.len()] {
	  self.command_sent = true;
	  message.get_responder()
	    .expect("Should have responder")
	    .respond(MessageContent::from_content(COMMAND_RESPONSE.to_string()))
	    .unwrap();
	}
      },
    };

    Ok(())
  }

  fn list_message_types(&self) -> Vec<MessageType> {
    vec!(
      MessageType::command_from_str(COMMAND_ID),
      MessageType::query_from_str(QUERY_ID),
    )
  }

  fn receive_client(&mut self, _client: SessionClient) {
    todo!()
  }
}
