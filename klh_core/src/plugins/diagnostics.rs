use crate::plugin::{Plugin, PluginListener};

pub(crate) struct Diagnostics;

impl Diagnostics {
  pub(crate) fn new() -> Self {
    Diagnostics{}
  }
}

impl Plugin for Diagnostics {
    fn accept_event(&self) -> Result<(), String> {
        todo!()
    }

    fn clone_listener(&self) -> Result<crate::plugin::PluginListener, String> {
        todo!()
    }

    fn list_events(&self) -> Vec<crate::event::Event> {
        todo!()
    }

    fn receive_client(&self, dispatch_client: crate::dispatch::DispatchClient) {
        todo!()
    }
}
