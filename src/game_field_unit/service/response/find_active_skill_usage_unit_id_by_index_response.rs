use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindActiveSkillUsageUnitIdByIndexResponse {
    found_unit_id: i32
}

impl FindActiveSkillUsageUnitIdByIndexResponse {
    pub fn new(found_unit_id: i32) -> Self {
        FindActiveSkillUsageUnitIdByIndexResponse { found_unit_id }
    }

    pub fn get_found_unit_id(&self) -> i32 {
        self.found_unit_id
    }
}
