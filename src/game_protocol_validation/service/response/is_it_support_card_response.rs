use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IsItSupportCardResponse {
    is_success: bool,
}

impl IsItSupportCardResponse {
    pub fn new(is_success: bool) -> Self {
        IsItSupportCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
