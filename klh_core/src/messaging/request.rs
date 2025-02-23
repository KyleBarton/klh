use tokio::sync::oneshot;

use super::{
    CommandMessage, EventMessage, Message, MessageContent, MessageError, MessageType, Responder,
    ResponseHandler,
};

/// The fundamental struct with which to communicate through klh. A
/// `Request` can apply to any [MessageType]. A user interfaces with
/// klh by constructing a `Request` instance, optionally obtaining the
/// response handler with [Request::get_handler], and then passing the
/// `Request` instance to [KlhClient::send](crate::klh::KlhClient).
/// # Examples
/// ```no_build,no_run
/// use klh_core::messaging::{Request, MessageType, MessageContent, MessageError};
/// use klh_core::klh::Klh;
///
/// let mut request = Request::new(
///   MessageType::command_from_str("plugin_name::command_name"),
///   MessageContent::from_content("Content"),
/// );
///
/// let klh_client = Klh::new().get_client();
///
/// let mut handler = request.get_handler();
///
/// klh_client.send(request).await.unwrap();
///
/// let response: Result<MessageContent, MessageError> = handler.handle_response().await;
/// ```
pub struct Request {
    message_type: MessageType,
    sender: Option<Responder>,
    receiver: Option<ResponseHandler>,
    content: Option<MessageContent>,
}

impl Request {
    /// Creates a request with a given [MessageType] and [MessageContent].
    pub fn new(message_type: MessageType, content: MessageContent) -> Self {
        match message_type {
            MessageType::Command(_) => {
                let (tx, rx) = oneshot::channel();
                Self {
                    message_type,
                    sender: Some(Responder::new(Some(tx))),
                    receiver: Some(ResponseHandler::new(Some(rx))),
                    content: Some(content),
                }
            }
            MessageType::Event(_) => Self {
                message_type,
                sender: None,
                receiver: None,
                content: Some(content),
            },
        }
    }

    /// Creates a request with a given [MessageType]. The request will
    /// hold no [MessageContent] data. This is useful for creating
    /// requests for whom the MessageType communicates all the relevant
    /// information (e.g. certain global events, such as
    /// `ShutDownRequested`).
    pub fn from_message_type(message_type: MessageType) -> Self {
        let (tx, rx) = oneshot::channel();
        Self {
            message_type,
            sender: Some(Responder::new(Some(tx))),
            receiver: Some(ResponseHandler::new(Some(rx))),
            content: None,
        }
    }

    pub(crate) fn as_message(&mut self) -> Message {
        match self.message_type {
            MessageType::Command(_) => Message::Command(CommandMessage::new(
                self.message_type,
                self.sender.take(),
                self.content.take(),
            )),
            MessageType::Event(_) => {
                Message::Event(EventMessage::new(self.message_type, self.content.take()))
            }
        }
    }

    /// A one-time-use method that will return the [ResponseHandler] for
    /// this request.
    /// # Errors
    /// Will return an error if the handler has already been taken from
    /// the request.
    pub fn get_handler(&mut self) -> Result<ResponseHandler, MessageError> {
        match self.receiver.take() {
            None => Err(MessageError::ResponseHandlerAlreadyTaken),
            Some(r) => Ok(r),
        }
    }
}

#[cfg(test)]
mod request_tests {
    use rstest::*;

    use crate::messaging::{Message, MessageContent, MessageError, MessageType};

    use super::Request;

    #[rstest]
    fn should_serialize_into_message_with_no_content() {
        let mut request: Request =
            Request::from_message_type(MessageType::command_from_str("cmd").unwrap());

        let mut serialized_message = request.as_message();

        assert_eq!(serialized_message.get_content(), None);
    }

    #[rstest]
    fn should_serialize_into_message_with_expected_content() {
        let mut request: Request = Request::new(
            MessageType::command_from_str("command").unwrap(),
            MessageContent::from_content("content"),
        );

        let mut serialized_message = request.as_message();

        assert_eq!(
            serialized_message.get_content(),
            Some(MessageContent::from_content("content"))
        );
    }

    #[rstest]
    fn should_serialize_into_message_with_command_message_type() {
        let mut request: Request =
            Request::from_message_type(MessageType::command_from_str("command").unwrap());

        assert!(!request.sender.is_none());
        assert!(!request.receiver.is_none());
        let serialized_message = request.as_message();
        assert!(matches!(serialized_message, Message::Command(..)));
        assert_eq!(
            serialized_message.get_message_type(),
            MessageType::command_from_str("command").unwrap(),
        );
    }

    #[rstest]
    fn should_serialize_into_message_with_event_message_type() {
        let mut request: Request = Request::new(
            MessageType::event_from_str("event").unwrap(),
            MessageContent::from_content("this is content"),
        );

        assert!(request.sender.is_none());
        assert!(request.receiver.is_none());
        let serialized_message = request.as_message();
        assert!(matches!(serialized_message, Message::Event(..)));
        assert_eq!(
            serialized_message.get_message_type(),
            MessageType::event_from_str("event").unwrap(),
        )
    }

    #[rstest]
    fn should_provide_handler_when_get_handler_first_called() {
        let mut request: Request =
            Request::from_message_type(MessageType::command_from_str("command").unwrap());

        let handler = request.get_handler();

        assert!(handler.is_ok())
    }

    #[rstest]
    fn should_return_expected_err_when_get_handler_called_more_than_once() {
        let mut request: Request =
            Request::from_message_type(MessageType::command_from_str("command").unwrap());

        let _handler = request.get_handler();

        let handler = request.get_handler();

        assert_eq!(
            handler.unwrap_err(),
            MessageError::ResponseHandlerAlreadyTaken
        )
    }
}
