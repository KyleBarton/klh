use tokio::sync::mpsc;


// TODO Placeholder. This needs to be thought out better.
// Let's replace this with event
// #[derive(Clone, Debug)]
// pub enum DispatchInput{
//   Test(String),
// }
use crate::event::Event;

pub(crate) struct Dispatcher;

// TODO Impl an async "send" api
pub(crate) struct DispatchClient {
  transmitter: mpsc::Sender<Event>,
}


// Needs work
impl DispatchClient {
  pub(crate) async fn send(&self, event: Event) -> Result<(), mpsc::error::SendError<Event>> {
    self.transmitter.send(event).await
  }
}

// Needs its own file/module. Needs to implement clone/copy? At least Clone.
pub(crate) struct DispatchOptions {
  input_receiver: Option<mpsc::Receiver<Event>>,
  input_transmitter: mpsc::Sender<Event>,
}

impl DispatchOptions {
  pub(crate) fn new() -> Self {
    let (tx, rx) = mpsc::channel(128);
    Self {
      input_receiver: Some(rx),
      input_transmitter: tx,
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
      input_transmitter: self.input_transmitter.clone()
    }
  }
}

impl Clone for DispatchOptions {
    fn clone(&self) -> Self {
      Self {
	input_receiver: None,
	input_transmitter: self.input_transmitter.clone(),
      }
    }
}


// Needs to be its own file/module. Pure functional
impl Dispatcher {

  pub(crate) async fn start_listener(options: DispatchOptions) -> Result<(), String> {
    let mut receiver = match options.input_receiver {
      Some(r) => r,
      None => return Err(String::from("Sender not authorized to start listener"))
    };
    while let Some(input) = receiver.recv().await {
      tokio::spawn(async move {
	println!("Received input {:?}!", input)
      });
    }

    Ok(())
  }

  pub(crate) fn get_client(options: DispatchOptions) -> Result<DispatchClient, String> {
    Ok(DispatchClient{
      transmitter: options.input_transmitter.clone(),
    })
  }
}
