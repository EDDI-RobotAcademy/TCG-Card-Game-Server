use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeApplyDamageToSpecificUnitByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeApplyDamageToSpecificUnitByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeApplyDamageToSpecificUnitByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}