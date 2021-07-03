use tokio::sync::mpsc;


// TODO Placeholder. This needs to be thought out better.
#[derive(Copy, Clone, Debug)]
pub struct DispatchInput;

pub(crate) struct Dispatch;

// TODO Impl an async "send" api
pub(crate) struct DispatchClient {
  transmitter: mpsc::Sender<DispatchInput>,
}


// Needs work
impl DispatchClient {
  pub(crate) async fn send(&self, input: DispatchInput) -> Result<(), mpsc::error::SendError<DispatchInput>> {
    self.transmitter.send(input).await
  }
}

// Needs its own file/module. Needs to implement clone/copy? At least Clone.
pub(crate) struct DispatchOptions {
  input_receiver: Option<mpsc::Receiver<DispatchInput>>,
  input_transmitter: mpsc::Sender<DispatchInput>,
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
impl Dispatch {

  pub(crate) async fn start_listener(options: DispatchOptions) -> Result<(), String> {
    let mut receiver = match options.input_receiver {
      Some(r) => r,
      None => return Err(String::from("Sender not authorized to start listener"))
    };
    tokio::spawn(async move {
      while let Some(input) = receiver.recv().await {
	println!("Received input {:?}!", input);
      }
    });
    Ok(())
  }

  pub(crate) fn get_client(options: DispatchOptions) -> Result<DispatchClient, String> {
    Ok(DispatchClient{
      transmitter: options.input_transmitter.clone(),
    })
  }
}
