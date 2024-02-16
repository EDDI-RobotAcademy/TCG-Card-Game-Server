use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseDamageMainCharacterItemCardResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseDamageMainCharacterItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseDamageMainCharacterItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}