use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}