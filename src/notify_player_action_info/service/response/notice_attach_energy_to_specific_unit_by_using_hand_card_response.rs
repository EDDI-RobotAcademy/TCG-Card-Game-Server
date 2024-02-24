use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}