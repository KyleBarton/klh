use core::panic;

use tokio::sync::mpsc;

use crate::{messaging::Message, plugin::{PluginChannel, PluginRegistrar}};

use log::debug;


#[derive(Clone)]
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
  plugin_registrar: PluginRegistrar,
}

impl Dispatch {
  pub(crate) fn new() -> Self {
    let (tx, rx) = mpsc::channel(128);
    Self {
      input_receiver: Some(rx),
      input_transmitter: tx,
      plugin_registrar: PluginRegistrar::new(),
    }
  }
  pub(crate) fn is_uncloned(&self) -> bool {
    match &self.input_receiver {
      Some(_) => true,
      None => false,
    }
  }

  // A special clone that takes the receiver
  pub(crate) fn clone_once(&mut self) -> Self {
    let input_receiver = match self.input_receiver.take() {
      Some(r) => Some(r),
      None => panic!("Cannot clone a clone"),
    };
    self.input_receiver = None;

    Self {
      input_receiver,
      input_transmitter: self.input_transmitter.clone(),
      plugin_registrar: self.plugin_registrar.clone(),
    }
  }

  // TODO error handling
  async fn dispatch_to_plugin(&self, message: Message) -> Result<(), String> {
    self.plugin_registrar.send_to_plugin(message).await;
    Ok(())
  }

  pub(crate) fn register_plugin(&mut self, plugin_channel: &PluginChannel) -> Result<(), String> {
    match self.plugin_registrar.register_plugin_message_types(plugin_channel) {
      Err(msg) => Err(msg),
      Ok(_) => Ok(()),
    }
  }

  pub(crate) fn get_client(&self) -> Result<DispatchClient, String> {
    Ok(DispatchClient::new(self.input_transmitter.clone()))
  }

  pub(crate) async fn start_listener(&mut self) -> Result<(), String> {
    let mut receiver = match self.input_receiver.take() {
      Some(r) => r,
      None => {
	return Err("Dispatch is already used.".to_string());
      },
    };
    while let Some(msg) = receiver.recv().await {
      let thread_dispatch = self.clone();
      tokio::spawn(async move {
	match thread_dispatch.dispatch_to_plugin(msg).await {
	  Ok(_) => Ok(()),
	  Err(msg) => Err(msg),
	}
      });
    }
    Ok(())
  }
}

impl Clone for Dispatch {
    fn clone(&self) -> Self {
      Self {
	input_receiver: None,
	input_transmitter: self.input_transmitter.clone(),
	plugin_registrar: self.plugin_registrar.clone(),
      }
    }
}

