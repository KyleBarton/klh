#[derive(Debug, Eq, PartialEq)]
pub enum CorePlugins {
  Buffers,
  Diagnostics,
  Displays,
}
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
