use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingActiveSkillResponseForm {
    is_success: bool,
}

impl TargetingActiveSkillResponseForm {
    pub fn new(is_success: bool) -> Self { TargetingActiveSkillResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}
