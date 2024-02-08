use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CanUseCardResponse {
    is_success: bool,
}

impl CanUseCardResponse {
    pub fn new(is_success: bool) -> Self {
        CanUseCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
