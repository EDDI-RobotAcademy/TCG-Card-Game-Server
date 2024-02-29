use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseSearchUnitFromDeckSupportCardResponse {
    is_success: bool,
}

impl NoticeUseSearchUnitFromDeckSupportCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseSearchUnitFromDeckSupportCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}