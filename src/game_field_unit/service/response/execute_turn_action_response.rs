use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteTurnActionResponse {
    is_success: bool
}

impl ExecuteTurnActionResponse {
    pub fn new(is_success: bool) -> Self {
        ExecuteTurnActionResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}