use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeTargetingAttackActiveSkillToUnitResponse {
    is_success: bool,
}

impl NoticeTargetingAttackActiveSkillToUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeTargetingAttackActiveSkillToUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}