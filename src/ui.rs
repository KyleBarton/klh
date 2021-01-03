use iced_native::{keyboard, layout, text_input, event, Widget,
		  Hasher, Layout, Point, Clipboard, Length,
		  Rectangle};
use iced::{Application, Element, Column, Container, Align, executor, Command, futures, Subscription};
use crate::models::{ControlType, InputType};
use text_input::{State, Value};

//TODO rename
pub struct IncomingEventProcessor {
  receiver: crossbeam_channel::Receiver<UiUpdate>,
}

pub enum ReceiverState {
  Open(crossbeam_channel::Receiver<UiUpdate>),
  Closed
}

impl<H,I> iced_native::subscription::Recipe<H,I> for IncomingEventProcessor
where H: std::hash::Hasher
{
  type Output = UiUpdate;
  //TODO not sure what to do with this
  fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        // self.url.hash(state);
  }

  fn stream(
    self: Box<Self>,
    _input: futures::stream::BoxStream<'static, I>
  ) -> futures::stream::BoxStream<'static, Self::Output> {
    // let receiver_clone = self.receiver.clone();
    Box::pin(futures::stream::unfold(
      ReceiverState::Open(self.receiver.clone()),
      |state| async move {
	match state {
	  ReceiverState::Closed => None,
	  ReceiverState::Open(rx) => {
	    // TODO error handling should be better here
	    let update = rx.recv().unwrap();
	    Some((update, ReceiverState::Open(rx.clone())))
	  }
	}
      }
    ))
  }
}
// Hopefully we can just extend this method from now on to interpret key inputs
// TODO controlType is leaky, "save" should not be known to the UI
// This is a leftover from when we were using termion::Key for the actual input
pub fn to_outbound_message(event: event::Event, modifiers: keyboard::Modifiers) -> Option<UiMessage> {
  if modifiers.control {
    match event {
      event::Event::Keyboard(keyboard::Event::KeyPressed { key_code, ..}) => {
	match key_code {
	  keyboard::KeyCode::S => Some(UiMessage::Outbound(InputType::Control(ControlType::Save))),
	  _ => None,
	}
      },
      _ => None,
    }
  }
  else {
    match event {
      event::Event::Keyboard(keyboard::Event::CharacterReceived(c)) => {
	Some(UiMessage::Outbound(InputType::Insert(c)))
      },
      event::Event::Keyboard(keyboard::Event::KeyPressed { key_code, ..}) => {
	match key_code {
	  keyboard::KeyCode::Left => Some(UiMessage::Outbound(InputType::Control(ControlType::ArrowLeft))),
	  keyboard::KeyCode::Right => Some(UiMessage::Outbound(InputType::Control(ControlType::ArrowRight))),
	  _ => None,
	}
      }
      _ => None,
    }
  }
}

pub struct BufferInput<Message>{
  content: String,
  modifiers: keyboard::Modifiers,
  to_message: fn(event::Event, keyboard::Modifiers) -> Option<Message>,
} 

impl BufferInput<UiMessage> {
  fn new(
    content: &str,
    on_change: fn(event::Event, keyboard::Modifiers) -> Option<UiMessage>,
  ) -> Self
  {
    Self {
      content: content.to_string(),
      modifiers: keyboard::Modifiers {
	shift: false,
	control: false,
	alt: false,
	logo: false,
      },
      to_message: on_change,
    }
  }
}

impl<Message, Renderer> Widget<Message, Renderer> for BufferInput<Message>
where Renderer: text_input::Renderer
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
    match event {
      event::Event::Keyboard(keyboard::Event::ModifiersChanged(mods)) => {
	self.modifiers = mods;
      },
      _ => {
	match (self.to_message)(
	  event, self.modifiers) {
	  Some(msg) => messages.push(msg),
	  None => (),
	};
      }
    }
    event::Status::Ignored
  }

  fn draw(
    &self,
    renderer: &mut Renderer,
    _defaults: &Renderer::Defaults,
    layout: Layout<'_>,
    cursor_position: Point,
    _viewport: &Rectangle,) -> Renderer::Output {
    let mut state = State::focused();
    state.move_cursor_to_end();
    text_input::Renderer::draw(
      renderer,
      layout.bounds(),
      layout.bounds(),
      cursor_position,
      Renderer::Font::default(),
      renderer.default_size(),
      "",
      &Value::new(&self.content),
      &state, //TODO should be a field
      &Renderer::Style::default(),
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
  ui_rx: crossbeam_channel::Receiver<UiUpdate>,
}

#[derive(Debug, Clone)]
pub struct Flags {
  session_tx: crossbeam_channel::Sender<InputType>,
  ui_rx: crossbeam_channel::Receiver<UiUpdate>,
}

impl Flags {
  pub fn new(
    session_tx: crossbeam_channel::Sender<InputType>,
    ui_rx: crossbeam_channel::Receiver<UiUpdate>
  ) -> Self {
    Flags {
      session_tx,
      ui_rx,
    }
  }
}

// Probably oversimplified right now
#[derive(Debug)]
pub enum UiUpdate {
  ContentRedisplay(String),
  Waiting,
}

#[derive(Debug)]
pub enum UiMessage {
  Outbound(InputType),
  Inbound(UiUpdate),
}

impl Application for EditorUi {
  type Message = UiMessage;
  type Flags = Flags;
  type Executor = executor::Default;

  fn new(flags: Flags) -> (Self, Command<Self::Message>) {
    (
      EditorUi {
	content: String::from("contents! Let there be contents!"),
	session_tx: flags.session_tx,
	ui_rx: flags.ui_rx,
      },
      Command::none()
    )
  }

  fn title(&self) -> String {
    String::from("What are you doing fix this")
  }

  fn subscription(&self) -> Subscription<Self::Message> {
    Subscription::from_recipe(IncomingEventProcessor {
      receiver: self.ui_rx.clone(),
    }).map(UiMessage::Inbound)
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      UiMessage::Outbound(input_type) => {
	match input_type {
	  InputType::Insert(c) => {
	    // TODO should probably use a subscription instead of directly pushing
	    // self.content.push(c);
	    ();
	  }
	  _ => {
	    ();
	  }
	}
	self.session_tx.send(input_type).unwrap();
      },
      UiMessage::Inbound(ui_update) => {
	match ui_update {
	  UiUpdate::ContentRedisplay(conts) => {
	    self.content = conts;
	  },
	  UiUpdate::Waiting => (),
	}
      }
    };
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
	to_outbound_message,
      ));

    Container::new(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x()
      .center_y()
      .into()
  }
}
