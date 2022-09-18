use serde::{Serialize, Deserialize};

/// Collection of known error identifiers that can be returned in the [messaging](super) module.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum MessageError {
  ResponseAlreadyHandled,
  MessageTypeNotFound,
  FailedToReceiveResponse,
  /// Indicates that the associated [Request](super::Request) object
  /// has already given away ownership of its
  /// [ResponseHandler](super::ResponseHandler). This error will be
  /// shown if [Request::get_handler](super::Request::get_handler) is
  /// called on the same struct instance more than once.
  ResponseHandlerAlreadyTaken,
  /// Indicates that the `RequestResponder` has already responded to
  /// the request one the oneshot channel. Since this is a one-time
  /// action, subsequent calls to
  /// [RequestResponder::respond](super::RequestResponder::respond)
  /// after the first will result in an [Err] of this type.
  RequestResponderAlreadyUsed,
}
