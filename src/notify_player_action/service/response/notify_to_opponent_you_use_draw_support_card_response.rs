use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseDrawSupportCardResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseDrawSupportCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseDrawSupportCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}