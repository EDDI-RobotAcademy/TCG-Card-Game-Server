use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseCatastrophicDamageItemCardResponse {
    is_success: bool,
}

impl NoticeUseCatastrophicDamageItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseCatastrophicDamageItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}