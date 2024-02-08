use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseEnergyBoostCardResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseEnergyBoostCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseEnergyBoostCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
