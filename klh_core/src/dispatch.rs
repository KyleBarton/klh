use core::panic;

use tokio::sync::mpsc;

use crate::{event::EventMessage, plugin::{PluginRegistrar, PluginTransmitter}};

pub(crate) struct Dispatcher;

pub(crate) struct DispatchClient {
  transmitter_v2: mpsc::Sender<EventMessage>,
}

impl DispatchClient {
  pub(crate) fn new(transmitter: mpsc::Sender<EventMessage>) -> Self {
    Self {
      transmitter_v2: transmitter,
    }
  }
}


// Needs work
impl DispatchClient {

  pub(crate) async fn send_v2(&self, event_message: EventMessage) -> Result<(), mpsc::error::SendError<EventMessage>> {
    println!("Sending event message: {}", event_message);
    self.transmitter_v2.send(event_message).await
  }
}

// Needs its own file/module. Needs to implement clone/copy? At least Clone.
pub(crate) struct Dispatch {
  input_receiver_v2: Option<mpsc::Receiver<EventMessage>>,
  input_transmitter_v2: mpsc::Sender<EventMessage>,
  plugin_registrar: PluginRegistrar,
}

impl Dispatch {
  pub(crate) fn new() -> Self {
    let (tx_v2, rx_v2) = mpsc::channel(128);
    Self {
      input_receiver_v2: Some(rx_v2),
      input_transmitter_v2: tx_v2,
      plugin_registrar: PluginRegistrar::new(),
    }
  }
  pub(crate) fn is_uncloned(&self) -> bool {
    match &self.input_receiver_v2 {
      Some(_) => true,
      None => false,
    }
  }

  // A special clone that takes the receiver
  pub(crate) fn clone_once(&mut self) -> Self {
    let input_receiver_v2 = match self.input_receiver_v2.take() {
      Some(r) => Some(r),
      None => panic!("Cannot clone a clone"),
    };
    self.input_receiver_v2 = None;

    Self {
      input_receiver_v2,
      input_transmitter_v2: self.input_transmitter_v2.clone(),
      plugin_registrar: self.plugin_registrar.clone(),
    }
  }

  pub(crate) fn register_plugin(&mut self, plugin_transmitter: PluginTransmitter) -> Result<(), String> {
    match self.plugin_registrar.register_plugin_event_types(plugin_transmitter) {
      Err(msg) => Err(msg),
      Ok(_) => Ok(()),
    }
  }

  pub(crate) fn get_client(&self) -> Result<DispatchClient, String> {
    Ok(DispatchClient::new(self.input_transmitter_v2.clone()))
  }

  // TODO error handling
  pub(crate) async fn dispatch_to_plugin_v2(&self, event_message: EventMessage) -> Result<(), String> {
    self.plugin_registrar.send_to_plugin_v2(event_message).await;
    Ok(())
  }
}

impl Clone for Dispatch {
    fn clone(&self) -> Self {
      Self {
	input_receiver_v2: None,
	input_transmitter_v2: self.input_transmitter_v2.clone(),
	plugin_registrar: self.plugin_registrar.clone(),
      }
    }
}

// Needs to be its own file/module. Pure functional
impl Dispatcher {

  pub(crate) async fn start_listener(mut dispatch: Dispatch) -> Result<(), String> {
    let mut receiver = match dispatch.input_receiver_v2.take() {
      Some(r) => r,
      None => return Err(String::from("Sender not authorized to start listener"))
    };
    while let Some(input) = receiver.recv().await {
      let thread_dispatch = dispatch.clone();
      tokio::spawn(async move {
	match thread_dispatch.dispatch_to_plugin_v2(input).await {
	  Ok(_) => Ok(()),
	  Err(msg) => Err(msg)
	}
      });
    }

    Ok(())
  }
}
