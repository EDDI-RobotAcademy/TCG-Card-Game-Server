use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeTurnStartTargetingAttackToGameMainCharacterResponse {
    is_success: bool,
}

impl NoticeTurnStartTargetingAttackToGameMainCharacterResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeTurnStartTargetingAttackToGameMainCharacterResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}