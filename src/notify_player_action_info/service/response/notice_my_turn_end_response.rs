use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeMyTurnEndResponse {
    is_success: bool,
}

impl NoticeMyTurnEndResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeMyTurnEndResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}