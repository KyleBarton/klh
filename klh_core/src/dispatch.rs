use tokio::sync::mpsc;

use crate::{event::Event, plugin::{PluginRegistrar, PluginTransmitter}};

pub(crate) struct Dispatcher;

// TODO Impl an async "send" api
pub struct DispatchClient {
  transmitter: mpsc::Sender<Event>,
}


// Needs work
impl DispatchClient {
  pub(crate) async fn send(&self, event: Event) -> Result<(), mpsc::error::SendError<Event>> {
    self.transmitter.send(event).await
  }
}

// Needs its own file/module. Needs to implement clone/copy? At least Clone.
pub(crate) struct Dispatch {
  input_receiver: Option<mpsc::Receiver<Event>>,
  input_transmitter: mpsc::Sender<Event>,
  plugin_registrar: PluginRegistrar,
}

// TODO this is where all the real stuff happens. Maybe just call this Dispatch?
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

  pub(crate) fn register_plugin(&mut self, plugin_transmitter: PluginTransmitter) -> Result<(), String> {
    match self.plugin_registrar.register_plugin_events(plugin_transmitter) {
      Err(msg) => Err(msg),
      Ok(_) => Ok(()),
    }
  }

  pub(crate) fn get_client(&self) -> Result<DispatchClient, String> {
    Ok(DispatchClient{
      transmitter: self.input_transmitter.clone(),
    })
  }

  // TODO error handling
  pub(crate) async fn dispatch_to_plugin(&self, event: Event) -> Result<(), String> {
    self.plugin_registrar.send_to_plugin(event).await;
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

// Needs to be its own file/module. Pure functional
impl Dispatcher {

  pub(crate) async fn start_listener(mut dispatch: Dispatch) -> Result<(), String> {
    let mut receiver = match dispatch.input_receiver.take() {
      Some(r) => r,
      None => return Err(String::from("Sender not authorized to start listener"))
    };
    while let Some(input) = receiver.recv().await {
      let thread_dispatch = dispatch.clone();
      tokio::spawn(async move {
	match thread_dispatch.dispatch_to_plugin(input).await {
	  Ok(_) => Ok(()),
	  Err(msg) => Err(msg)
	}
      });
    }

    Ok(())
  }

  pub(crate) fn get_client(options: Dispatch) -> Result<DispatchClient, String> {
    // TODO I can just use clone() here right?
    Ok(DispatchClient{
      transmitter: options.input_transmitter.clone(),
    })
  }
}
