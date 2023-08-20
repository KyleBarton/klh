## Messaging module

This module contains types & structs necessary to serialize and send information through a running Klh instance. Important types include:

- `Request`: the fundamental DTO for sending information through Klh
- `Responder`: The type used to send informaton back to a requester -
  used for queries
- `Message`: The type which a `Plugin` implementation receives from
  the running Klh instance, containing the information from a sent
  request.
- `MessageType`: The enum that captures types of commands, queries,
  and events that are registered in the Klh system.

