use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployNonTargetingAttackPassiveSkillResponseForm {
    is_success: bool,
}

impl DeployNonTargetingAttackPassiveSkillResponseForm {
    pub fn new(is_success: bool) -> Self { DeployNonTargetingAttackPassiveSkillResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}