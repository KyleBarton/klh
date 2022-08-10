use crate::{event::EventMessage, session::{Session, SessionClient}};

pub struct KlhClient {
  session_client: SessionClient,
}

impl KlhClient {
  pub fn new(session_client: SessionClient) -> Self {
    Self {
      session_client,
    }
  }

  pub async fn send(&mut self, event_message: EventMessage) -> Result<(), String> {
    self.session_client.send(event_message).await
  }
}

pub struct Klh {
  session: Session,
}

impl Klh {
  pub fn new() -> Self {
    let session = Session::new();
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
