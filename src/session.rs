use std::io;
use std::fs::File;
use std::io::Write;
use std::fmt::Write as other;
use crate::display;
use crate::input_handler;
use crate::startup;
use crate::buffer;
use crate::buffer_provider;
use crate::command_executor;
use crate::models::Command;

use termion::screen;

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

//thin wrapper over screen for now. Eventually should be the port to a proper display
pub struct SessionScreen {
    screen: screen::AlternateScreen<std::io::Stdout>,
}

impl SessionScreen {
    pub fn new() -> Result<SessionScreen, String> {
        //ugh the coupling here
        let screen = screen::AlternateScreen::from(std::io::stdout());
        Ok(SessionScreen {
            screen,
        })
    }

    pub fn display(&mut self, buffer: &impl buffer::Buffer) -> Result<(), String> {
        display::display_buffer_v2(buffer, &mut self.screen);
        Ok(())
    }
}

pub struct Session {
    startup_args: startup::StartupArgs,
    current_buffer: buffer::LineBuffer,
    state: SessionState,
    screen: SessionScreen,
    reader: std::io::Stdin, //todo eeeewww
}

impl Session {
    //TODO this is where you would new-up the buffer store
    //TODO This actually maybe should be distinct from start_up()
    pub fn new(args: startup::StartupArgs) -> Session {
        Session {
            startup_args: args,
            current_buffer: buffer_provider::new(buffer_provider::BufferType::Normal).unwrap(),
            state: SessionState::New,
            screen: SessionScreen::new().unwrap(),
            reader: std::io::stdin(),
        }
    }

    //Should only ever be run once for the session. Must bookend with a tear_down call
    fn init(&mut self) -> Result<(), String> {
        //redundant with constructor?
        self.screen = SessionScreen::new().unwrap();
        self.reader = std::io::stdin();
        self.state = SessionState::PostInit;

        let new_buffer = match self.startup_args.get_file_name() {
            None => buffer_provider::new(buffer_provider::BufferType::Normal).unwrap(),
            Some(f) => buffer_provider::from_file(buffer_provider::BufferType::Normal, f).unwrap(),
        };
        self.current_buffer = new_buffer;
        Ok(())
    }

    //This is where you can potentially load dynamic code, and initialize pub-sub
    //This is also where a "soft reload" would reset the sesion to
    //note that that means we have to assume buffers exist and they have to stay intact
    fn add_hooks(&mut self) -> Result<(), String> {
        self.state = SessionState::PostHooks;
        Ok(())
    }

    //This is where user/system config can be loaded
    fn load_config(&mut self) -> Result<(), String> {
        self.state = SessionState::UserReady;
        Ok(())
    }

    //await an input to act on
    fn await_user(&mut self, mut log: &mut impl std::fmt::Write) -> Result<(), String> {
        self.screen.display(&self.current_buffer).unwrap();
        let input = input_handler::await_input_v2(&mut self.reader, &mut log).unwrap();
        let command: Command = input_handler::process_input_v2(input, &mut log).unwrap();
        match command_executor::execute_command_v2(&command, &mut self.current_buffer, &mut log) {
            Some(_exit_code) => self.state = SessionState::TearDownReady,
            None => (),
        };
        Ok(())
    }

    //Can only be run once for the session. Must bookend with init()
    fn tear_down(&mut self, log: &String) -> Result<(), String> {
        let log_filename = "klh.log";
        Session::save_log(&log_filename, &log).unwrap();
        self.state = SessionState::PostTearDown;
        Ok(())
    }


    //I think this can be our only public function (other than new)
    pub fn run(&mut self) -> Result<(), String> {
        //TODO we'll keep the log in run for now and revamp logging completely later
        let mut log = String::from("");
        loop {
            match &self.state {
                SessionState::New => self.init(),
                SessionState::PostInit => self.add_hooks(),
                SessionState::PostHooks => self.load_config(),
                SessionState::UserReady => self.await_user(&mut log),
                SessionState::TearDownReady => { self.tear_down(&log) },
                SessionState::PostTearDown => break,
            }.unwrap();
        }
        Ok(())
    }

    fn save_log(filename: &str, log: &String) -> io::Result<()> {
        let mut file = File::create(filename)?;
        write!(&mut file, "{}", &log).unwrap();
        Ok(())
    }
}
