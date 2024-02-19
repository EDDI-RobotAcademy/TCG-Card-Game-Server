use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSupportCardUsageCountResponse {
    used_count: i32
}

impl CheckSupportCardUsageCountResponse {
    pub fn new(used_count: i32) -> Self {
        CheckSupportCardUsageCountResponse { used_count }
    }

    pub fn get_used_count(&self) -> i32 {
        self.used_count
    }
}