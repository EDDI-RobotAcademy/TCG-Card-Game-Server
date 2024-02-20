use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetTurnActionOfAllFieldUnitResponse {
    is_success: bool
}

impl ResetTurnActionOfAllFieldUnitResponse {
    pub fn new(is_success: bool) -> Self {
        ResetTurnActionOfAllFieldUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}