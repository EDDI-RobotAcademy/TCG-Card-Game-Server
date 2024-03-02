use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseSpecialEnergyCardToUnitResponse {
    is_success: bool,
}

impl NoticeUseSpecialEnergyCardToUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseSpecialEnergyCardToUnitResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}