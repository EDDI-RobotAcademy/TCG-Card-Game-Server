use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IsItEnergyCardResponse {
    is_success: bool,
}

impl IsItEnergyCardResponse {
    pub fn new(is_success: bool) -> Self {
        IsItEnergyCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
