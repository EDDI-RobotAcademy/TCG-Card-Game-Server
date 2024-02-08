use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseEnergyCardResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseEnergyCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseEnergyCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
