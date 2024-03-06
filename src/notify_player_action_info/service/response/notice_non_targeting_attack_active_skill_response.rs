use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeNonTargetingAttackActiveSkillResponse {
    is_success: bool,
}

impl NoticeNonTargetingAttackActiveSkillResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeNonTargetingAttackActiveSkillResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}