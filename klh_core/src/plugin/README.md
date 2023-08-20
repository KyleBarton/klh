## plugin module

The plugin module contains common code needed to implement and register plugins.

### Public API
The only contribution to the public Klh API in this module is the
`Plugin` trait, which represents the foundational strategy that a Klh
plugin must implement.


### Private
`PluginChannel` is the internal data plane: it allows `Session`s to
asynchronously dispatch requests to registered plugins. It is the
internal implementation of how Klh "starts" a plugin

`PluginRegistrar` is the internal control plane: it allows a `Session`
to map `MessageType`s to plugins, and send instances of those message
types along to the appropriate plugins accordingly.
