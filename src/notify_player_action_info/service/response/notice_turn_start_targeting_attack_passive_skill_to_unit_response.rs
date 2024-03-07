use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeTurnStartTargetingAttackPassiveSkillToUnitResponse {
    is_success: bool,
}

impl NoticeTurnStartTargetingAttackPassiveSkillToUnitResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeTurnStartTargetingAttackPassiveSkillToUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}