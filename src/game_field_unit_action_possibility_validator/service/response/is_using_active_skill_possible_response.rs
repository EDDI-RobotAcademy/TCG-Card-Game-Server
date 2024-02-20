use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsUsingActiveSkillPossibleResponse {
    is_possible: bool,
}

impl IsUsingActiveSkillPossibleResponse {
    pub fn new(is_possible: bool) -> Self { IsUsingActiveSkillPossibleResponse { is_possible } }
    pub fn is_possible(&self) -> bool { self.is_possible }
}