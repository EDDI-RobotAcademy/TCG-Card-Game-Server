use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeInstantDeathOfSpecificUnitByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeInstantDeathOfSpecificUnitByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeInstantDeathOfSpecificUnitByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}