## Plugins Module

This module is the current location for core KLH plugins. This is not
intended to be their final destination. This module is just a grab-bag
of these plugins until I find time to split them off from the `klh_core`
crate and into their own `klh_core_plugins` crate (or something like that)


Plugins that exist so far:

### Diagnostics
A plugin for sending test messages through a running klh
instance. Primarily used by me to manually test and exercise the
asynchronous KLH runtime.


### Buffers
The plugin that will control the editing and enumeration of active
buffers in Klh. Takes no dependencies on any other plugin.
