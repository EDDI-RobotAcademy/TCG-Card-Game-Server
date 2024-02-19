use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetSupportCardUsageCountResponse {
    is_success: bool
}

impl ResetSupportCardUsageCountResponse {
    pub fn new(is_success: bool) -> Self {
        ResetSupportCardUsageCountResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}