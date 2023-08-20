## Session

A session represents an instance of a KLH runtime. `SessionClient`
will operate very similarly to `KlhClient`, and work occur in the
future to remove session from the public API.


A session consists of:
- A collection of plugins listening on `PluginChannel` instances living in their own threads
- A `PluginRegistrar` instance which maps `MessageType`s to plugin transmitters
- A `Dispatch` instance, which will run the central listener and send requests along via the `PluginRegistrar`
