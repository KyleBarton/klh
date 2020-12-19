use crate::buffer_provider;
use crate::command_executor;
use crate::input_handler;
use crate::models::{InputType, Command};
use crate::{buffer_store::{BufferStoreArgs, BufferStore}, startup, ui::UiUpdate};
use log::*;

#[derive(Copy, Clone)]
enum SessionState {
  //Brand new, no state whatsoever
  New,
  //After init
  PostInit,
  //Reserving here for a place to dynamically load code (if possible)
  PostHooks,
  UserReady,
  TearDownReady,
  //Must be the last SessionState
  PostTearDown,
}

//thin wrapper over screen for now. Eventually should be the port to a
// proper display TODO at some point, I need an internal
// representation of the screen instead of just letting iced work
pub struct SessionScreen;

pub struct Session {
  startup_args: startup::StartupArgs,
  buffer_store: BufferStore,
  state: SessionState,
  receiver: crossbeam_channel::Receiver<InputType>,
  ui_tx: crossbeam_channel::Sender<UiUpdate>,
}

impl Session {
  pub fn new(
    args: startup::StartupArgs,
    receiver: crossbeam_channel::Receiver<InputType>,
    ui_tx: crossbeam_channel::Sender<UiUpdate>,
  ) -> Session {
    info!("Creating new session");
    Session {
      startup_args: args,
      buffer_store: BufferStore::new(),
      state: SessionState::New,
      receiver,
      ui_tx,
    }
  }

  //TODO this is where you would new-up the buffer store
  //Should only ever be run once for the session. Must bookend with a tear_down call
  fn init(&mut self) -> Result<(), &str> {
    self.state = SessionState::PostInit;

    match self.startup_args.get_file_name() {
      None => {
	let buffer_store_args = BufferStoreArgs::new(buffer_provider::BufferType::Normal, "");
	self.buffer_store.add_new(buffer_store_args);
      },
      Some(f) => {
	let buffer_store_args = BufferStoreArgs::new(buffer_provider::BufferType::Normal, f);
	self.buffer_store.add_new(buffer_store_args);
      }
    }
    Ok(())
  }

  //This is where you can potentially load dynamic code, and initialize pub-sub
  //This is also where a "soft reload" would reset the sesion to
  //note that that means we have to assume buffers exist and they have to stay intact
  fn add_hooks(&mut self) -> Result<(), &str> {
    self.state = SessionState::PostHooks;
    info!("Hooks loaded");
    Ok(())
  }

  //This is where user/system config can be loaded
  fn load_config(&mut self) -> Result<(), &str> {
    self.state = SessionState::UserReady;
    info!("Config loaded");
    Ok(())
  }

  //await an input to act on
  fn await_user(&mut self) -> Result<(), &str> {

    match self.buffer_store.get_current_mut() {
      Err(message) => Err(message),
      Ok(b) => {
	let content: String = b.get_chars().unwrap();
	self.ui_tx.send(UiUpdate::ContentRedisplay(content)).unwrap();
	let input = self.receiver.recv().unwrap();
	let command: Command = input_handler::process_input_v2(input).unwrap();
	match command_executor::execute_command_v2(&command, b) {
	    Some(_exit_code) => self.state = SessionState::TearDownReady,
	    None => (),
	};
	Ok(())
      }
    }

  }

  //Can only be run once for the session. Must bookend with init()
  fn tear_down(&mut self) -> Result<(), &str> {
    self.state = SessionState::PostTearDown;
    info!("Teardown complete");
    Ok(())
  }

  //I think this can be our only public function (other than new)
  pub fn run(&mut self) -> Result<(), &str> {
    info!("Starting session");
    loop {
      match &self.state {
        SessionState::New => self.init(),
        SessionState::PostInit => self.add_hooks(),
        SessionState::PostHooks => self.load_config(),
        SessionState::UserReady => self.await_user(),
        SessionState::TearDownReady => self.tear_down(),
        SessionState::PostTearDown => break,
      }
      .unwrap();
    }
    Ok(())
  }
}
