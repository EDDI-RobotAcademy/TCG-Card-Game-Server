use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitDamageInfo {
    field_unit_damage_map: HashMap<i32, i32>,
}

impl FieldUnitDamageInfo {
    pub fn new(field_unit_damage_map: HashMap<i32, i32>) -> Self {
        FieldUnitDamageInfo {
            field_unit_damage_map
        }
    }
}