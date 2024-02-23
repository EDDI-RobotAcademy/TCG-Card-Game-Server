use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeApplyDamageToEveryUnitByUsingHandCardResponse {
    is_success: bool,
}

impl NoticeApplyDamageToEveryUnitByUsingHandCardResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeApplyDamageToEveryUnitByUsingHandCardResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }
}