use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IsItUnitCardResponse {
    is_success: bool,
}

impl IsItUnitCardResponse {
    pub fn new(is_success: bool) -> Self {
        IsItUnitCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
