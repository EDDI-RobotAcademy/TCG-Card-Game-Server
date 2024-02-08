use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachSingleEnergyToUnitIndexResponse {
    is_success: bool
}

impl AttachSingleEnergyToUnitIndexResponse {
    pub fn new(is_success: bool) -> Self {
        AttachSingleEnergyToUnitIndexResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
