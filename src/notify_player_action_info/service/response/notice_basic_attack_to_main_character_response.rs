use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeBasicAttackToMainCharacterResponse {
    is_success: bool,
}

impl NoticeBasicAttackToMainCharacterResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeBasicAttackToMainCharacterResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}