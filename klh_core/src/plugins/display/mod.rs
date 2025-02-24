use log::{debug, error, warn};
use models::{AcceptStringInputRequest, AttachBufferRequest, GetWindowRequest, GetWindowResponse};

use crate::{
    messaging::{CommandMessage, Message, MessageContent, MessageError, MessageType},
    plugin::Plugin,
    plugins::{buffers, display::models::CreateWindowRequest},
    session::SessionClient,
};

use self::models::Window;

pub mod models;
pub mod requests;

pub struct Displays {
    message_types: Vec<MessageType>,
    // TODO move to klh client!
    session_client: Option<SessionClient>,
    windows: Vec<Window>,
}

impl Displays {
    pub fn new() -> Self {
        let message_types: Vec<MessageType> = vec![
            MessageType::command_from_str("display::create_window").unwrap(),
            MessageType::command_from_str("display::list_windows").unwrap(),
            MessageType::command_from_str("display::get_window").unwrap(),
            MessageType::command_from_str("display::delete_window").unwrap(),
            MessageType::command_from_str("display::attach_buffer").unwrap(),
            MessageType::command_from_str("display::detach_buffer").unwrap(),
            MessageType::command_from_str("display::list_buffers_in_window").unwrap(),
            MessageType::command_from_str("display::accept_string_input").unwrap(),
        ];

        Self {
            message_types,
            session_client: None,
            windows: Vec::new(),
        }
    }

    fn accept_command(&mut self, mut message: CommandMessage) -> Result<(), MessageError> {
        debug!("[DISPLAY] received command message {:?}", message);
        let message_type = message.get_message_type();

        if message_type.id_equals_str("display::list_windows") {
            let response = models::ListWindowsResponse {
                window_names: self
                    .windows
                    .iter()
                    .map(|window| window.name.clone())
                    .collect(),
            };
            if let Err(e) = message
                .get_responder()
                .expect("No one should have used responder yet")
                .respond(MessageContent::from_content(response))
            {
                error!("[DISPLAY] failed to respond to message. Error: {:?}", e);
                return Err(MessageError::PluginFailedToProcessMessage);
            }
        } else if message_type.id_equals_str("display::create_window") {
            let mut request_content = message.get_content().expect("Got a window name");
            let request: CreateWindowRequest = request_content.deserialize().unwrap();

            if self
                .windows
                .iter()
                .any(|window| window.name == request.window_name)
            {
                return Err(MessageError::BadRequest(String::from(
                    "Window name already exists",
                )));
            }

            self.windows.push(Window::new(request.window_name));
        } else if message_type.id_equals_str("display::attach_buffer") {
            let mut request_content = message.get_content().expect("Got a window name");
            let request: AttachBufferRequest = request_content.deserialize().unwrap();
            let window: &mut Window = self
                .windows
                .iter_mut()
                .find(|window| window.name == request.window_name)
                .expect("Found window");

            // TODO check with buffers plugin to ensure it's an actual buffer name
            window
                .associated_buffers_names
                .push(request.buffer_name.clone());
            // Assume this becomes the active buffer when this happens
            // Ok, what if I just re-order the vec? Idk figure this out later
            window.active_buffer_name = Some(request.buffer_name)
        } else if message_type.id_equals_str("display::get_window") {
            let mut request_content = message.get_content().expect("Got a window name");
            let request: GetWindowRequest = request_content.deserialize().unwrap();
            let window = self
                .windows
                .iter()
                .find(|window| window.name == request.window_name);

            let response = GetWindowResponse {
                window: window.cloned(),
            };
            message
                .get_responder()
                .expect("No one should have used responder yet")
                .respond(MessageContent::from_content(response))
                .unwrap();
        }
        //My first inter-plugin command!
        else if message_type.id_equals_str("display::accept_string_input") {
            let mut request_content = message.get_content().expect("Got a window name");
            let request: AcceptStringInputRequest = request_content.deserialize().unwrap();
            let window = match self
                .windows
                .iter()
                .find(|window| window.name == request.window_name)
            {
                Some(window) => window,
                None => {
                    return Err(MessageError::BadRequest(String::from(
                        "Could not find window by provided name",
                    )));
                }
            };

            let active_buffer_name = match &window.active_buffer_name {
                Some(b) => b.clone(),
                // TODO this should be possible later (e.g. when opening a buffer)
                None => {
                    return Err(MessageError::BadRequest(String::from(
                        "Cannot accept input in a window without an active buffer",
                    )));
                }
            };

            let mut request_to_buffer = buffers::requests::new_append_string_to_buffer_request(
                &active_buffer_name,
                &request.input,
            );

            //Ugh this presents a huge problem
            // - How do I make this method async?
            // - Gotta move to klh client
            // - Can I just do this with a tokio spawn for now?
            // - Ok seems to work as is - let's test

            if let Some(client) = &self.session_client {
                let cli_clone = client.clone();
                tokio::spawn(async move {
                    cli_clone
                        .send(request_to_buffer.as_message())
                        .await
                        .unwrap();
                });
            };
        } else {
            warn!(
                "[DISPLAY] message type id not found: {}",
                &message.get_message_type()
            );
        }

        Ok(())
    }
}

impl Default for Displays {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for Displays {
    fn accept_message(&mut self, message: Message) -> Result<(), MessageError> {
        match message {
            Message::Event(_) => todo!(),
            Message::Command(command) => self.accept_command(command),
        }
    }

    fn list_message_types(&self) -> Vec<MessageType> {
        self.message_types.clone()
    }

    fn receive_client(&mut self, client: SessionClient) {
        self.session_client = Some(client);
    }
}
