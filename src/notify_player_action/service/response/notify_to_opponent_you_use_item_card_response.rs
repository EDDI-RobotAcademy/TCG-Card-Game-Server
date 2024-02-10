use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseItemCardResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
