use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseDrawSupportCardResponse {
    is_success: bool,
}

impl NoticeUseDrawSupportCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseDrawSupportCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}