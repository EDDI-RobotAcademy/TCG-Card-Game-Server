use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseFieldEnergyIncreaseItemCardResponse {
    is_success: bool,
}

impl NoticeUseFieldEnergyIncreaseItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseFieldEnergyIncreaseItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}