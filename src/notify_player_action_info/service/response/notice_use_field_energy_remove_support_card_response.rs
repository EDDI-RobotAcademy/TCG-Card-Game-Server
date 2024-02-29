use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseFieldEnergyRemoveSupportCardResponse {
    is_success: bool,
}

impl NoticeUseFieldEnergyRemoveSupportCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseFieldEnergyRemoveSupportCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}