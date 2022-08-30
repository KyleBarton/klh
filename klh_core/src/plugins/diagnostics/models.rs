use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct SlowBombContent {
  pub interval_seconds: u64,
}

