use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IsItItemCardResponse {
    is_success: bool,
}

impl IsItItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        IsItItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
