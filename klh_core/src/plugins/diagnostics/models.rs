use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SlowBombContent {
    pub interval_seconds: u64,
}
