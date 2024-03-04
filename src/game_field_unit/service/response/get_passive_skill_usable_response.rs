use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPassiveSkillUsableResponse {
    passive_skill_usable_list: Vec<bool>
}

impl GetPassiveSkillUsableResponse {
    pub fn new(passive_skill_usable_list: Vec<bool>) -> Self {
        GetPassiveSkillUsableResponse { passive_skill_usable_list }
    }

    pub fn get_passive_skill_usable_list(self) -> Vec<bool> {
        self.passive_skill_usable_list
    }
}