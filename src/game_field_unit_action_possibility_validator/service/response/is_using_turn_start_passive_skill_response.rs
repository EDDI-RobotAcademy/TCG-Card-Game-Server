use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsUsingTurnStartPassiveSkillPossibleResponse {
    is_possible: bool,
}

impl IsUsingTurnStartPassiveSkillPossibleResponse {
    pub fn new(is_possible: bool) -> Self { IsUsingTurnStartPassiveSkillPossibleResponse { is_possible } }
    pub fn is_possible(&self) -> bool { self.is_possible }
}