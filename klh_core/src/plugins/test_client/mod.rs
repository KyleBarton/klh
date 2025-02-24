use log::debug;

use crate::{messaging::MessageType, plugin::Plugin, session};

#[derive(Default)]
pub struct TestClient {
    client: Option<session::SessionClient>,
}

impl TestClient {
    pub fn new() -> Self {
        Self { client: None }
    }
}

impl Plugin for TestClient {
    fn accept_message(
        &mut self,
        message: crate::messaging::Message,
    ) -> Result<(), crate::messaging::MessageError> {
        debug!("[TESTCLIENT] Received message {}", message);
        Ok(())
    }

    fn list_message_types(&self) -> Vec<crate::messaging::MessageType> {
        vec![MessageType::event_from_str("buffers:buffer_appended").unwrap()]
    }

    fn receive_client(&mut self, client: crate::session::SessionClient) {
        self.client = Some(client);
    }
}
