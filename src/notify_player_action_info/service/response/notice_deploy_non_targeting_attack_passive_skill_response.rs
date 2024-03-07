use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeDeployNonTargetingAttackPassiveSkillResponse {
    is_success: bool,
}

impl NoticeDeployNonTargetingAttackPassiveSkillResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeDeployNonTargetingAttackPassiveSkillResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}