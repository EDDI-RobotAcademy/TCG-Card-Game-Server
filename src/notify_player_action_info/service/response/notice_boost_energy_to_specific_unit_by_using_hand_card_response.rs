use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}