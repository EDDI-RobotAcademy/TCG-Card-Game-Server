use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseEnergyBoostSupportCardToSpecificUnitResponse {
    is_success: bool,
}

impl NoticeUseEnergyBoostSupportCardToSpecificUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseEnergyBoostSupportCardToSpecificUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}