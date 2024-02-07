use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachMultipleEnergyToFieldUnitResponse {
    is_success: bool
}

impl AttachMultipleEnergyToFieldUnitResponse {
    pub fn new(is_success: bool) -> Self {
        AttachMultipleEnergyToFieldUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}