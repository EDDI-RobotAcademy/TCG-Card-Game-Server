use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitDeathInfo {
    dead_field_unit_index_list: Vec<i32>,
}

impl FieldUnitDeathInfo {
    pub fn new(dead_field_unit_index_list: Vec<i32>) -> Self {
        FieldUnitDeathInfo {
            dead_field_unit_index_list
        }
    }
    pub fn get_dead_field_unit_index_list(&self) -> &Vec<i32> {
        &self.dead_field_unit_index_list
    }
}