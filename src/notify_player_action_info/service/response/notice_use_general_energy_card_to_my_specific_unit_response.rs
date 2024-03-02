use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseGeneralEnergyCardToMySpecificUnitResponse {
    is_success: bool,
}

impl NoticeUseGeneralEnergyCardToMySpecificUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseGeneralEnergyCardToMySpecificUnitResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}