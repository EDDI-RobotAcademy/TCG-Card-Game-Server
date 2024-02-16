use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckIsUnitAliveResponse {
    is_alive: bool
}

impl CheckIsUnitAliveResponse {
    pub fn new(is_alive: bool) -> Self {
        CheckIsUnitAliveResponse { is_alive }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}
