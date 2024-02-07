use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupportCardProtocolValidationResponse {
    is_success: bool,
}

impl SupportCardProtocolValidationResponse {
    pub fn new(is_success: bool) -> Self {
        SupportCardProtocolValidationResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
