use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeSearchCardByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeSearchCardByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeSearchCardByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}