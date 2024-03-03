use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeBasicAttackToUnitResponse {
    is_success: bool,
}

impl NoticeBasicAttackToUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeBasicAttackToUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}