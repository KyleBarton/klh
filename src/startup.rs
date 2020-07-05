
#[derive(Debug)]
pub struct StartupArgs {
    file_name: String,
}

impl StartupArgs {
    pub fn new() -> StartupArgs {
        StartupArgs {
            file_name: String::from(""),
        }
    }
    //TODO Hacky assumes filename is the next arg
    pub fn from_cli() -> StartupArgs {
        let mut args = std::env::args();
        args.next();
        match args.next() {
            None => StartupArgs::new(),
            Some(f) => StartupArgs {
                file_name: String::from(&f),
            }
        }
    }

    pub fn get_file_name(&self) -> Option<&str>{
        if self.file_name == String::from("") {
            None
        } else {
            Some(&self.file_name)
        }
    }
}
