use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyInstantDeathToTargetUnitIndexResponse {
    is_success: bool
}

impl ApplyInstantDeathToTargetUnitIndexResponse {
    pub fn new(is_success: bool) -> Self {
        ApplyInstantDeathToTargetUnitIndexResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
