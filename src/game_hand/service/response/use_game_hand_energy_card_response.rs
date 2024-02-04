use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandEnergyCardResponse {
    is_success: bool,
}

impl UseGameHandEnergyCardResponse {
    pub fn new(is_success: bool) -> Self {
        UseGameHandEnergyCardResponse { is_success }
    }
}
