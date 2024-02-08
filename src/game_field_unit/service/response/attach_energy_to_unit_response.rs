use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachEnergyToUnitResponse {
    is_success: bool
}

impl AttachEnergyToUnitResponse {
    pub fn new(is_success: bool) -> Self {
        AttachEnergyToUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}