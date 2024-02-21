use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeDeathOfEveryUnitResponse {
    dead_unit_id_list: Vec<i32>
}

impl JudgeDeathOfEveryUnitResponse {
    pub fn new(dead_unit_id_list: Vec<i32>) -> Self {
        JudgeDeathOfEveryUnitResponse { dead_unit_id_list }
    }

    pub fn get_dead_unit_id_list(&self) -> &Vec<i32> {
        &self.dead_unit_id_list
    }
}
