use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseEnergyBoostSupportCardToMySpecificUnitResponse {
    is_success: bool,
}

impl NoticeUseEnergyBoostSupportCardToMySpecificUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseEnergyBoostSupportCardToMySpecificUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}