use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingPassiveSkillResponseForm {
    is_success: bool,
}

impl TargetingPassiveSkillResponseForm {
    pub fn new(is_success: bool) -> Self { TargetingPassiveSkillResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}
