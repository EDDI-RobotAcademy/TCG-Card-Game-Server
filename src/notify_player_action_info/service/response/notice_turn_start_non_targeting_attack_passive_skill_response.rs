use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeTurnStartNonTargetingAttackPassiveSkillResponse {
    is_success: bool,
}

impl NoticeTurnStartNonTargetingAttackPassiveSkillResponse {
    pub fn new(is_success: bool) -> Self {
        NoticeTurnStartNonTargetingAttackPassiveSkillResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}