use klh::session;
use klh::startup::StartupArgs;
use simplelog;
use std::fs;
use iced::{self, Application, };
use klh::ui::{Flags, EditorUi};
use std::thread;
use crossbeam_channel;

fn main() -> iced::Result {
  let args: StartupArgs = StartupArgs::from_cli();

  let (session_tx, session_rx) = crossbeam_channel::unbounded();

  thread::spawn(move || {
    /*setting up some logging*/
    simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
      simplelog::LevelFilter::Info,
      simplelog::Config::default(),
      fs::File::create("klh.log").unwrap(),
    )])
    .unwrap();
    /*end*/
    let mut session: session::Session = session::Session::new(args, session_rx);
    session.run().unwrap();
  });

  
  EditorUi::run(iced::Settings::with_flags(
    Flags::new(session_tx)
  ))
}
