use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseFieldEnergyRemoveItemCardResponse {
    is_success: bool,
}

impl NoticeUseFieldEnergyRemoveItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseFieldEnergyRemoveItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}