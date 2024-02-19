use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeDeathOfUnitResponse {
    dead_unit_id: i32
}

impl JudgeDeathOfUnitResponse {
    pub fn new(dead_unit_id: i32) -> Self {
        JudgeDeathOfUnitResponse { dead_unit_id }
    }

    pub fn get_dead_unit_id(&self) -> i32 {
        self.dead_unit_id
    }
}
