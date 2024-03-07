use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeDeployTargetingAttackToGameMainCharacterResponse {
    is_success: bool,
}

impl NoticeDeployTargetingAttackToGameMainCharacterResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeDeployTargetingAttackToGameMainCharacterResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}