use crate::{event::EventMessage, session::{SessionOptions, Session, SessionClient}};

pub struct KlhClient {
  session_client: SessionClient,
}

impl KlhClient {
  pub fn new(session_client: SessionClient) -> Self {
    Self {
      session_client,
    }
  }

  pub async fn send_v2(&mut self, event_message: EventMessage) -> Result<(), String> {
    self.session_client.send_v2(event_message).await
  }
}

pub struct Klh {
  session: Session,
}

impl Klh {
  pub fn new() -> Self {
    let session_options = SessionOptions::new();
    let session = Session::new(session_options);
    Self {
      session,
    }
  }

  pub async fn start(&mut self) {
    self.session.run().await.unwrap();
    println!("Session started");
    
  }

  pub fn get_client(&self) -> Result<KlhClient, String> {
    let client = KlhClient::new(self.session.get_client().unwrap());
    Ok(client)
  }
}
