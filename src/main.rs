use klh::session;
use klh::startup::StartupArgs;
use simplelog;
use std::fs;
use std::io;
use termios::*;

fn main() -> io::Result<()> {
  /*setting up some logging*/
  simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
    simplelog::LevelFilter::Info,
    simplelog::Config::default(),
    fs::File::create("klh.log").unwrap(),
  )])
  .unwrap();
  /*end*/
  let std_fd = libc::STDIN_FILENO;
  /*These two lines have to stay together*/
  let given_termios = Termios::from_fd(std_fd).unwrap();
  let mut new_termios = given_termios.clone();
  set_term_raw(&mut new_termios, std_fd).unwrap();
  /*END*/

  let args: StartupArgs = StartupArgs::from_cli();

  let mut session: session::Session = session::Session::new(args);

  session.run().unwrap();

  reset_term(given_termios, std_fd);

  Ok(())
}

fn set_term_raw(mut term: &mut Termios, fd: i32) -> Result<(), String> {
  termios::cfmakeraw(&mut term);
  //we're not doing anything fancy with display yet, we just need raw input
  term.c_oflag |= OPOST;
  Ok(tcsetattr(fd, TCSANOW, &term).unwrap())
}

fn reset_term(term: Termios, fd: i32) {
  tcsetattr(fd, TCSANOW, &term).unwrap();
}
