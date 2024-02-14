use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseToolCardToEnhanceAttackPointResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseToolCardToEnhanceAttackPointResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseToolCardToEnhanceAttackPointResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}