use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeDeathOfEveryUnitResponse {
    dead_unit_tuple_list: Vec<(i32, i32)>
}

impl JudgeDeathOfEveryUnitResponse {
    pub fn new(dead_unit_tuple_list: Vec<(i32, i32)>) -> Self {
        JudgeDeathOfEveryUnitResponse { dead_unit_tuple_list }
    }

    pub fn get_dead_unit_index_list(&self) -> Vec<i32> {
        let mut dead_unit_index_list = Vec::new();
        for (index, id) in &self.dead_unit_tuple_list {
            if *id != -1 {
                dead_unit_index_list.push(*index)
            }
        }
        dead_unit_index_list
    }
    pub fn get_dead_unit_id_list(&self) -> Vec<i32> {
        let mut dead_unit_id_list = Vec::new();
        for (_, id) in &self.dead_unit_tuple_list {
            if *id != -1 {
                dead_unit_id_list.push(*id)
            }
        }
        dead_unit_id_list
    }
}
