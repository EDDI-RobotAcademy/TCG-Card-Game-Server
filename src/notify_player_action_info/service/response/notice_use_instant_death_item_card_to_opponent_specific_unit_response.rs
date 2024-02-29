use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseInstantDeathItemCardToOpponentSpecificUnitResponse {
    is_success: bool,
}

impl NoticeUseInstantDeathItemCardToOpponentSpecificUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseInstantDeathItemCardToOpponentSpecificUnitResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}