use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyDamageToTargetUnitIndexResponse {
    is_success: bool
}

impl ApplyDamageToTargetUnitIndexResponse {
    pub fn new(is_success: bool) -> Self {
        ApplyDamageToTargetUnitIndexResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
