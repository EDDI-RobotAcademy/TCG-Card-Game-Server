use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyToOpponentYouUseItemInstantDeathAlternativesResponse {
    is_success: bool,
}

impl NotifyToOpponentYouUseItemInstantDeathAlternativesResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyToOpponentYouUseItemInstantDeathAlternativesResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
