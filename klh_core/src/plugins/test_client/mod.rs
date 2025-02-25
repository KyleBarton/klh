use log::debug;

use crate::{klh::KlhClient, messaging::MessageType, plugin::Plugin};

#[derive(Default)]
pub struct TestClient {
    client: Option<KlhClient>,
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

    fn receive_client(&mut self, client: KlhClient) {
        self.client = Some(client);
    }
}
