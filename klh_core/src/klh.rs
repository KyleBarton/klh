use crate::{event::Event, session::{SessionOptions, Session, SessionClient}};

//klh.rs.
pub struct KlhClient {
  session_client: SessionClient,
}

impl KlhClient {
  pub fn new(session_client: SessionClient) -> Self {
    Self {
      session_client,
    }
  }
  pub async fn send(&mut self, event: Event) -> Result<(), String> {
    self.session_client.send(event).await
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

  // TODO probably don't need to make these async, since the thread runtimes handle it
  pub async fn start(&mut self) {
    // println!("Starting plugins");
    // // TODO probably don't need to make these async, since the thread runtimes handle it
    // self.session.discover_plugins().await;
    println!("Plugins started. Starting session");
    self.session.run().await.unwrap();
    println!("Session started");
    
  }

  pub fn get_client(&self) -> Result<KlhClient, String> {
    let client = KlhClient::new(self.session.get_client().unwrap());
    Ok(client)
  }
}
