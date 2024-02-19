use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckFieldEnergyEnoughToUseResponse {
    is_success: bool,
}

impl CheckFieldEnergyEnoughToUseResponse {
    pub fn new(is_success: bool) -> Self { CheckFieldEnergyEnoughToUseResponse { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}