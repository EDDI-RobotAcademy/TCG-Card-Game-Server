use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseHandCardResponse {
    is_success: bool,
}

impl NoticeUseHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}