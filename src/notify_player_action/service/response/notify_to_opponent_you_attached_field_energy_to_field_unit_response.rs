use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentYouAttachedFieldEnergyResponse {
    is_success: bool,
}

impl NotifyOpponentYouAttachedFieldEnergyResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyOpponentYouAttachedFieldEnergyResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}