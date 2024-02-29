use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseInstantUnitDeathItemCardResponse {
    is_success: bool,
}

impl NoticeUseInstantUnitDeathItemCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeUseInstantUnitDeathItemCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}