use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsUsingDeployPassiveSkillPossibleResponse {
    is_possible: bool,
}

impl IsUsingDeployPassiveSkillPossibleResponse {
    pub fn new(is_possible: bool) -> Self { IsUsingDeployPassiveSkillPossibleResponse { is_possible } }
    pub fn is_possible(&self) -> bool { self.is_possible }
}