use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachMultipleEnergyToUnitIndexResponse {
    is_success: bool
}

impl AttachMultipleEnergyToUnitIndexResponse {
    pub fn new(is_success: bool) -> Self {
        AttachMultipleEnergyToUnitIndexResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
