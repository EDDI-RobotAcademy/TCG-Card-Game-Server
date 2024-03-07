use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeDeployTargetingAttackPassiveSkillToUnitResponse {
    is_success: bool,
}

impl NoticeDeployTargetingAttackPassiveSkillToUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeDeployTargetingAttackPassiveSkillToUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}