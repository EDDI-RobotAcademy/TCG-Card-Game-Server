use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCatastrophicDamageToFieldUnitResponse {
    is_success: bool
}

impl ApplyCatastrophicDamageToFieldUnitResponse {
    pub fn new(is_success: bool) -> Self {
        ApplyCatastrophicDamageToFieldUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}