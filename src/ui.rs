use iced_native::{keyboard, layout, text, text_input, event, Widget,
		  Hasher, Layout, Point, Clipboard, Length,
		  Rectangle, Color, HorizontalAlignment, VerticalAlignment};
use iced::{Application, Element, Column, Container, Align, executor, Command};
use crate::{input_handler, models::InputType};
// A UI, written as an Iced widget!

pub struct BufferInput<Message>{
  content: String,
  on_change: fn(char) -> Message,
} 

impl BufferInput<InputType> {
  fn new(
    content: &str,
    on_change: fn(char) -> InputType,
  ) -> Self
  {
    Self {
      content: content.to_string(),
      on_change,
    }
  }
}

impl<Message, Renderer> Widget<Message, Renderer> for BufferInput<Message>
where Renderer: text::Renderer
{
  // Just cover the entire space for now
  fn width(&self) -> Length {
    Length::Fill
  }

  // Just cover the entire space for now
  fn height(&self) -> Length {
    Length::Fill
  }

  fn layout(&self, _renderer: &Renderer, limits: &layout::Limits,) -> layout::Node {
    let node: layout::Node = layout::Node::new(limits.max());
    node
  }

  /*
  Computes the layout hash of the Widget.
  The produced hash is used by the runtime to decide if the Layout
  needs to be recomputed between frames. Therefore, to ensure maximum
  efficiency, the hash should only be affected by the properties of
  the Widget that can affect layouting.
  For example, the Text widget does not hash its color property, as its value cannot affect the overall Layout of the user interface.
  */
  fn hash_layout(&self, _state: &mut Hasher) {
    // Length::Shrink.hash(state); //TODO what even is this?
  }

  fn on_event(
	  &mut self,
	  event: event::Event,
	  _layout: Layout<'_>,
	  _cursor_position: Point,
	  messages: &mut Vec<Message>,
	  _renderer: &Renderer,
	  _clipboard: Option<&dyn Clipboard>,
  ) -> event::Status {
    // We're gonna have to key-press/key-release thing to get modifiers figured out
    match event {
      event::Event::Keyboard(keyboard::Event::CharacterReceived(c)) => {
	let message: Message = (self.on_change)(c);
	messages.push(message);
      },
      _ => {
	// IDK
      }
    }
    event::Status::Ignored
  }

  fn draw(
    &self,
    renderer: &mut Renderer,
    defaults: &Renderer::Defaults,
    layout: Layout<'_>,
    _cursor_position: Point,
    _viewport: &Rectangle,) -> Renderer::Output {
    Renderer::draw(
      renderer,
      defaults,
      layout.bounds(),
      &self.content,
      renderer.default_size(),
      Renderer::Font::default(),
      Some(Color::BLACK),
      HorizontalAlignment::Left,
      VerticalAlignment::Top,
    )
  }
  
}

impl<'a, Message, Renderer> Into<iced_native::Element<'a, Message, Renderer>> for BufferInput<Message>
where Renderer: text_input::Renderer,
Message: 'a,
{
  fn into(self) -> iced_native::Element<'a, Message, Renderer> {
    iced_native::Element::new(self)
  }
}

pub struct EditorUi {
  content: String,
  session_tx: crossbeam_channel::Sender<InputType>,
}

#[derive(Debug, Clone)]
pub struct Flags {
  session_tx: crossbeam_channel::Sender<InputType>,
}

impl Flags {
  pub fn new(session_tx: crossbeam_channel::Sender<InputType>) -> Self {
    Flags {
      session_tx,
    }
  }
}

impl Application for EditorUi {
  type Message = InputType;
  type Flags = Flags;
  type Executor = executor::Default;

  fn new(flags: Flags) -> (Self, Command<Self::Message>) {
    (
      EditorUi {
	content: String::from("contents! Let there be contents!"),
	session_tx: flags.session_tx,
      },
      Command::none()
    )
  }

  fn title(&self) -> String {
    String::from("What are you doing fix this")
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      InputType::Insert(c) => {
	// TODO should probably use a subscription instead of directly pushing
	self.content.push(c);
      }
      _ => {
	();
      }
    };
    self.session_tx.send(message).unwrap();
    Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    let content = Column::new()
      .padding(20)
      .spacing(20)
      .max_width(500)
      .align_items(Align::Center)
      .push(BufferInput::new(
	&self.content,
	input_handler::translate_char,
      ));

    Container::new(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x()
      .center_y()
      .into()
  }
}
