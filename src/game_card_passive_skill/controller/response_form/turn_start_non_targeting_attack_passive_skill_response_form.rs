use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnStartNonTargetingAttackPassiveSkillResponseForm {
    is_success: bool,
}

impl TurnStartNonTargetingAttackPassiveSkillResponseForm {
    pub fn new(is_success: bool) -> Self { TurnStartNonTargetingAttackPassiveSkillResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}