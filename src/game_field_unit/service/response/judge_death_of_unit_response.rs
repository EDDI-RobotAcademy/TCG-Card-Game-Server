use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeDeathOfUnitResponse {
    dead_unit_tuple: (i32, i32)
}

impl JudgeDeathOfUnitResponse {
    pub fn new(dead_unit_tuple: (i32, i32)) -> Self {
        JudgeDeathOfUnitResponse { dead_unit_tuple }
    }

    pub fn get_dead_unit_index(&self) -> i32 {
        self.dead_unit_tuple.0
    }

    pub fn get_dead_unit_id(&self) -> i32 {
        self.dead_unit_tuple.1
    }
}
