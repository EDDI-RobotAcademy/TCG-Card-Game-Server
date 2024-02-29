use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseSearchDeckSupportCardResponse {
    is_success: bool,
}

impl NoticeUseSearchDeckSupportCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseSearchDeckSupportCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}