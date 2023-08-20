use serde::{Serialize, Deserialize};

/// Collection of known error identifiers that can be returned in the [messaging](super) module.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum MessageError {
  /// Indicates that the associated
  /// [ResponseHandler](super::ResponseHandler) has already handled
  /// the response with
  /// [ResponseHandler::handle_response](super::ResponseHandler::handle_response). A
  /// ResponseHandler can only handle a query response once before
  /// being consumed.
  ResponseAlreadyHandled,
  /// Indicates that a message type is not yet registered with the KLH
  /// instance has not registed a plugin for this message type. This
  /// error means that KLH was sent a message that it did not know how
  /// to dispatch.
  MessageTypeNotFound,
  /// Indicates that a [ResponseHandler](super::ResponseHandler)
  /// attempted to handle a query response, but the response never
  /// came. Generally indicates that an error occurred in the plugin
  /// that was sent the request preventing a response from being sent.
  FailedToReceiveResponse,
  /// Indicates that the associated [Request](super::Request) object
  /// has already given away ownership of its
  /// [ResponseHandler](super::ResponseHandler). This error will be
  /// shown if [Request::get_handler](super::Request::get_handler) is
  /// called on the same struct instance more than once.
  ResponseHandlerAlreadyTaken,
  /// Indicates that the `Responder` has already responded to
  /// the request one the oneshot channel. Since this is a one-time
  /// action, subsequent calls to
  /// [Responder::respond](super::Responder::respond)
  /// after the first will result in an [Err] of this type.
  ResponderAlreadyUsed,
  /// Indicates that a message was sent to its plugin, but that the
  /// plugin was unable to process the message when
  /// [Plugin::accept_message](crate::plugin::Plugin::accept_message)
  /// was invoked. Points to an issue on the plugin side.
  PluginFailedToProcessMessage,
  /// Indicates that a [Responder](crate::messaging::Responder) was
  /// unable to send a response along the oneshot channel. This points
  /// to a serious problem in the request/response infrastructure.
  FailedToSendResponse,
}
