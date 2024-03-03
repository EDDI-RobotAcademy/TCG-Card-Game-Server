use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTargetingAttackPassiveSkillResponseForm {
    is_success: bool,
}

impl DeployTargetingAttackPassiveSkillResponseForm {
    pub fn new(is_success: bool) -> Self { DeployTargetingAttackPassiveSkillResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}
