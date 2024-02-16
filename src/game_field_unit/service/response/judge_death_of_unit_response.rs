use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeDeathOfUnitResponse {
    is_alive: bool
}

impl JudgeDeathOfUnitResponse {
    pub fn new(is_alive: bool) -> Self {
        JudgeDeathOfUnitResponse { is_alive }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}
