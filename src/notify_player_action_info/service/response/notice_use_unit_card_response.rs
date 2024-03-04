use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseUnitCardResponse {
    is_success: bool,
}

impl NoticeUseUnitCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseUnitCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}