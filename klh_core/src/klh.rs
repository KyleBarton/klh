use log::debug;

use crate::{messaging::Request, session::{Session, SessionClient}, plugin::Plugin};

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
    self.session_client.send(request.to_message().unwrap()).await
  }
}

pub struct Klh {
  session: Session,
  plugins: Option<Vec<Box<dyn Plugin + Send>>>,
}

impl Klh {
  pub fn new() -> Self {
    let session = Session::new();
    Self {
      session,
      plugins: None,
    }
  }

  pub async fn start(&mut self) {
    match self.plugins.take() {
      None => (),
      Some(plugin_list) => self.session.register_plugins(plugin_list),
    };
    self.session.run().await.unwrap();
    debug!("Session started successfully");
  }

  pub fn add_plugin(&mut self, plugin: Box<dyn Plugin + Send>) {
    match self.plugins.take() {
      None => self.plugins = Some(vec!(plugin)),
      Some(mut plugin_list) => {
	plugin_list.push(plugin);
	self.plugins = Some(plugin_list);
      }
    }
  }

  pub fn get_client(&self) -> KlhClient {
    // TODO eliminate this result chain
    let client = KlhClient::new(self.session.get_client().unwrap());
    client
  }
}
