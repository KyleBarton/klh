use tokio::sync::mpsc;

use crate::{messaging::Message, plugin::PluginRegistrar};

use log::debug;


#[derive(Clone, Debug)]
pub(crate) struct DispatchClient {
  transmitter: mpsc::Sender<Message>,
}

impl DispatchClient {
  pub(crate) fn new(transmitter: mpsc::Sender<Message>) -> Self {
    Self {
      transmitter,
    }
  }
}

// Needs work
impl DispatchClient {

  pub(crate) async fn send(&self, message: Message) -> Result<(), mpsc::error::SendError<Message>> {
    debug!("Sending message: {}", message);
    self.transmitter.send(message).await
  }
}

pub(crate) struct Dispatch {
  input_receiver: Option<mpsc::Receiver<Message>>,
  input_transmitter: mpsc::Sender<Message>,
}

impl Dispatch {
  pub(crate) fn new() -> Self {
    let (tx, rx) = mpsc::channel(128);
    Self {
      input_receiver: Some(rx),
      input_transmitter: tx,
    }
  }

  pub(crate) fn get_client(&self) -> Result<DispatchClient, String> {
    Ok(DispatchClient::new(self.input_transmitter.clone()))
  }

  pub(crate) async fn start_listener(
    &mut self,
    plugin_registrar: PluginRegistrar,
  ) -> Result<(), String> {
    let mut receiver = match self.input_receiver.take() {
      Some(r) => r,
      None => {
	return Err("Dispatch is already used.".to_string());
      },
    };
    while let Some(msg) = receiver.recv().await {
      debug!("Dispatch received message: {}", msg);
      // self.dispatch_to_plugin(msg).await.unwrap();
      plugin_registrar.send_to_plugin(msg).await
    }
    Ok(())
  }
}
