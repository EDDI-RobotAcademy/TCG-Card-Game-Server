use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetAllPassiveOfUnitResponse {
    is_success: bool
}

impl ResetAllPassiveOfUnitResponse {
    pub fn new(is_success: bool) -> Self {
        ResetAllPassiveOfUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}