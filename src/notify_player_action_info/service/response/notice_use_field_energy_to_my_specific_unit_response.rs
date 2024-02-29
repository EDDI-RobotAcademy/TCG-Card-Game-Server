use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseFieldEnergyToMySpecificUnitResponse {
    is_success: bool,
}

impl NoticeUseFieldEnergyToMySpecificUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseFieldEnergyToMySpecificUnitResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}