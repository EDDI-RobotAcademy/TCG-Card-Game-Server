use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseUnitEnergyRemoveItemCardResponse {
    is_success: bool,
}

impl NoticeUseUnitEnergyRemoveItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseUnitEnergyRemoveItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}