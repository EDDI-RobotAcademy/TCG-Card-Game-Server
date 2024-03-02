use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseMultipleUnitDamageItemCardResponse {
    is_success: bool,
}

impl NoticeUseMultipleUnitDamageItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseMultipleUnitDamageItemCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}