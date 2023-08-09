/// A list of core plugins with which to start Klh. This will likely
/// be refactored as configuration needs become more sophisticated.
#[derive(Debug, Eq, PartialEq)]
pub enum CorePlugins {
  Buffers,
  Diagnostics,
  Displays,
}

/// An object containing a representation of any configuration needed
/// to start an instance of [Klh](super::klh::Klh)
pub struct KlhConfig {
  pub core_plugins: Vec<CorePlugins>,
}

impl Default for KlhConfig {
  fn default() -> Self {
    Self {
      core_plugins: vec!(
	CorePlugins::Buffers,
	CorePlugins::Diagnostics,
	CorePlugins::Displays
      ),
    }
  }
}

impl KlhConfig {
  pub fn with_core_plugins(core_plugins: Vec<CorePlugins>) -> Self {
    Self {
      core_plugins,
    }
  }
  
}
