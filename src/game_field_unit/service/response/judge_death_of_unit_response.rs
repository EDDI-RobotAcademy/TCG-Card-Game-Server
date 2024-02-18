use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeDeathOfUnitResponse {
    is_dead: bool
}

impl JudgeDeathOfUnitResponse {
    pub fn new(is_dead: bool) -> Self {
        JudgeDeathOfUnitResponse { is_dead }
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }
}
