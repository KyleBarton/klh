#[derive(Copy, Clone, Debug)]
pub enum Command {
  BufferInsert(char),
  BufferDelete,
  AdvancePoint,
  RetreatPoint,
  Save,
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
  Save,
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum InputType {
  Waiting, //Initialized, not yet received user input
  Insert(char),
  Control(ControlType),
}
