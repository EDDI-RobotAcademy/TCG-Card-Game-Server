use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseDestroyDeckItemCardResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseDestroyDeckItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseDestroyDeckItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}