use std::fs;

use log::info;

use crate::{messaging::Request, session::{Session, SessionClient}};

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

  // TODO
  pub async fn send(&mut self, mut request: Request) -> Result<(), String> {
    self.session_client.send(request.to_message().unwrap()).await
  }
}

pub struct Klh {
  session: Session,
}

impl Klh {
  pub fn new() -> Self {

    simplelog::CombinedLogger::init(
      vec![
	simplelog::WriteLogger::new(
	  simplelog::LevelFilter::Debug,
	  simplelog::Config::default(),
	  fs::File::create("klh.log").unwrap(),
	)
      ]
    ).unwrap();

    let session = Session::new();
    Self {
      session,
    }
  }

  pub async fn start(&mut self) {
    self.session.run().await.unwrap();
    info!("Session started");
    
  }

  pub fn get_client(&self) -> Result<KlhClient, String> {
    let client = KlhClient::new(self.session.get_client().unwrap());
    Ok(client)
  }
}
