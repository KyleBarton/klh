use tokio::sync::mpsc;

use crate::{messaging::Message, plugin::PluginRegistrar};

use log::debug;

/*2023-08-07 - I'm documenting Dispatch with rustdoc even though it
 * will not be part of KLH's public API. It's a complex enough
 * structure that it warrants the same treatment.
 * So why Dispatch?
 * Here's one idea: the fact that dispatch can only start_listener
 * once, because of the nature of channels. A started, and then
 * stopped, dispatch is useless. That being said, `session` could
 * potentially start a whole new dispatch after the error (or run two
 * different dispatches in certain cases!). In other words, the fact
 * that dispatch holds the mpsc Receiver/Sender that is central to
 * KLH's communication is what makes it valuable as a different struct
 * from session. Another simple way to put it: dispatch, as
 * implemented here, is tied to tokio::sync::mpsc. Session should be
 * able to abstract that. */


/// Collection of errors related to the operation of
/// [Dispatch](Dispatch) & its provisioning of clients.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum DispatchError {
  /// Indicates that this instance of the [Dispatch](Dispatch) struct
  /// has already started and stopped. An instance of Dispatch cannot
  /// be run more than one time, before a new one must be created.
  DispatchAlreadyUsed,
}


/// Client that allows a user to send a [Message] to the instance of
/// dispatch that provided the client.
#[derive(Clone, Debug)]
pub(crate) struct DispatchClient {
  transmitter: mpsc::Sender<Message>,
}

impl DispatchClient {
  /// Create a new dispatch client from a transmitter.
  fn new(transmitter: mpsc::Sender<Message>) -> Self {
    Self {
      transmitter,
    }
  }
}

// Needs work
impl DispatchClient {

  /// Send a message through the transmitter. An instance of Dispatch
  /// should be listening on the other end.
  /// TODO test what happens when you try to send a message to a dispatch that isn't running.
  pub(crate) async fn send(&self, message: Message) -> Result<(), mpsc::error::SendError<Message>> {
    debug!("Sending message: {}", message);
    self.transmitter.send(message).await
  }
}

/// Dispatch is the struct that fundamentally owns the main execution
/// loop of KLH. It listens for messages on an mpsc channel and sends
/// them off to the plugin registrar to be sent to a KLH plugin for
/// processing.
pub(crate) struct Dispatch {
  /// The means by which Dispatch listens for messages. This field is
  /// an Option because Dispatch::start can only be called
  /// once. Afterwards, the mpsc receiver is used up, and this field
  /// becomes None. Any attempts to start the dispatch instance
  /// afterwards will produce a DispatchAlreadyUsed error.
  input_receiver: Option<mpsc::Receiver<Message>>,
  /// The means by which a message can be sent to a running
  /// Dispatch. Dispatch clones this transmitter as it vends
  /// DispatchClient instances.
  input_transmitter: mpsc::Sender<Message>,
}

impl Dispatch {
  /// Create a new instance of Dispatch
  pub(crate) fn new() -> Self {
    let (tx, rx) = mpsc::channel(128);
    Self {
      input_receiver: Some(rx),
      input_transmitter: tx,
    }
  }

  /// Provide a new instance of DispatchClient, which is instantiated
  /// to send to this instance of Dispatch when running.
  pub(crate) fn get_client(&self) -> DispatchClient {
    DispatchClient::new(self.input_transmitter.clone())
  }

  /// Start the Dispatch listening loop.
  pub(crate) async fn start(
    &mut self,
    plugin_registrar: PluginRegistrar,
  ) -> Result<(), DispatchError> {
    let mut receiver = match self.input_receiver.take() {
      Some(r) => r,
      None => {
	return Err(DispatchError::DispatchAlreadyUsed);
      },
    };
    while let Some(msg) = receiver.recv().await {
      debug!("Dispatch received message: {}", msg);
      plugin_registrar.send_to_plugin(msg).await
    }
    Ok(())
  }
}
