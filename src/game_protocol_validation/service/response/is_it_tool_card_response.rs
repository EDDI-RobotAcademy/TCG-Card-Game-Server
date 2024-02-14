use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IsItToolCardResponse {
    is_success: bool,
}

impl IsItToolCardResponse {
    pub fn new(is_success: bool) -> Self {
        IsItToolCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}