use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnStartTargetingAttackPassiveSkillResponseForm {
    is_success: bool,
}

impl TurnStartTargetingAttackPassiveSkillResponseForm {
    pub fn new(is_success: bool) -> Self { TurnStartTargetingAttackPassiveSkillResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}
