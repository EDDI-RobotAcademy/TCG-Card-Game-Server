use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquireUnitAttackPointResponse {
    is_success: bool
}

impl AcquireUnitAttackPointResponse {
    pub fn new(is_success: bool) -> Self {
        AcquireUnitAttackPointResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
