use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachSpecialEnergyToUnitIndexResponse {
    is_success: bool
}

impl AttachSpecialEnergyToUnitIndexResponse {
    pub fn new(is_success: bool) -> Self {
        AttachSpecialEnergyToUnitIndexResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
