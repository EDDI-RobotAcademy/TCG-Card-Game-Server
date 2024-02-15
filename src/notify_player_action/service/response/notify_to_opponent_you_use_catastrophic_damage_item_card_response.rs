use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseCatastrophicDamageItemCardResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseCatastrophicDamageItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseCatastrophicDamageItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}