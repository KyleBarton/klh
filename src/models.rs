pub struct ContentBuffer {
    pub content: String, //todo do I want a byte array here?
    pub point: i64, //Allowing negatives to keep the program from crashing. Point needs to be more thoroughly thought out
}

/*
Just `Editor` for now, but later can apply to display modifications, project management, etc
This may be better of as a string that gets looked up later for functions. Or something better.
TODO Dynamic function lookup in rust
  */
#[derive(Copy, Clone, Debug)]
pub enum Command {
    BufferInsert(char),
    BufferDelete,
    AdvancePoint,
    RetreatPoint,
    Quit,
    Default, //mostly for stubbing
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ControlType {
    Escape,
    Backspace,
    ArrowLeft,
    ArrowRight,
    CursorUp,
    CursorDown,
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum InputType {
    Waiting, //Initialized, not yet received user input
    Insert(char),
    Control(ControlType),
}
