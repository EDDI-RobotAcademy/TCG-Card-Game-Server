use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeDrawCardByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeDrawCardByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeDrawCardByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}