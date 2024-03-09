use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeTargetingAttackActiveSkillToGameMainCharacterResponse {
    is_success: bool,
}

impl NoticeTargetingAttackActiveSkillToGameMainCharacterResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeTargetingAttackActiveSkillToGameMainCharacterResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}