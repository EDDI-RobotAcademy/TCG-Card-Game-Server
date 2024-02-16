use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonTargetingActiveSkillResponseForm {
    is_success: bool,
}

impl NonTargetingActiveSkillResponseForm {
    pub fn new(is_success: bool) -> Self { NonTargetingActiveSkillResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}