use core::panic;

use tokio::sync::mpsc;

use crate::{event::EventMessage, plugin::{PluginRegistrar, PluginTransmitter}};


pub(crate) struct DispatchClient {
  transmitter: mpsc::Sender<EventMessage>,
}

impl DispatchClient {
  pub(crate) fn new(transmitter: mpsc::Sender<EventMessage>) -> Self {
    Self {
      transmitter,
    }
  }
}


// Needs work
impl DispatchClient {

  pub(crate) async fn send(&self, event_message: EventMessage) -> Result<(), mpsc::error::SendError<EventMessage>> {
    println!("Sending event message: {}", event_message);
    self.transmitter.send(event_message).await
  }
}

// Needs its own file/module. Needs to implement clone/copy? At least Clone.
pub(crate) struct Dispatch {
  input_receiver: Option<mpsc::Receiver<EventMessage>>,
  input_transmitter: mpsc::Sender<EventMessage>,
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
  async fn dispatch_to_plugin(&self, event_message: EventMessage) -> Result<(), String> {
    self.plugin_registrar.send_to_plugin(event_message).await;
    Ok(())
  }

  pub(crate) fn register_plugin(&mut self, plugin_transmitter: PluginTransmitter) -> Result<(), String> {
    match self.plugin_registrar.register_plugin_event_types(plugin_transmitter) {
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
    while let Some(event_msg) = receiver.recv().await {
      let thread_dispatch = self.clone();
      tokio::spawn(async move {
	match thread_dispatch.dispatch_to_plugin(event_msg).await {
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

