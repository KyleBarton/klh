use tokio::sync::oneshot;

use super::{Message, MessageType, MessageContent, Responder, ResponseHandler, MessageError};

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
    let (tx, rx) = oneshot::channel();
    Self {
      message_type,
      sender: Some(Responder::new(Some(tx))),
      receiver: Some(ResponseHandler::new(Some(rx))),
      content: Some(content),
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

  pub(crate) fn to_message(&mut self) -> Message {
    match self.content.take() {
      None => Message::new(
	self.message_type,
	self.sender.take(),
	None,
      ),
      Some(content) => Message::new(
	self.message_type,
	self.sender.take(),
	Some(content),
      ),
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

