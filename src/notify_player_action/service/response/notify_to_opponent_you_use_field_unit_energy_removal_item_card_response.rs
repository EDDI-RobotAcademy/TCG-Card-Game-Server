use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardResponse {
    is_success: bool,
}

impl NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}