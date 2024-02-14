use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachMultipleEnergyFromFieldUnitResponse {
    is_success: bool
}

impl DetachMultipleEnergyFromFieldUnitResponse {
    pub fn new(is_success: bool) -> Self {
        DetachMultipleEnergyFromFieldUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}