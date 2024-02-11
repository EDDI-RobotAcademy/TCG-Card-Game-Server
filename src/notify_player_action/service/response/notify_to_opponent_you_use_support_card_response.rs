use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentYouUseSupportCardResponse {
    is_success: bool,
}

impl NotifyOpponentYouUseSupportCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyOpponentYouUseSupportCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}