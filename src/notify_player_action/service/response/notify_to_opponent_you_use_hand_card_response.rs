use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentYouUseHandCardResponse {
    is_success: bool,
}

impl NotifyOpponentYouUseHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyOpponentYouUseHandCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}