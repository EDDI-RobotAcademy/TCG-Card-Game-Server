use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitHealthPointInfo {
    field_unit_health_point_map: HashMap<i32, i32>,
}

impl FieldUnitHealthPointInfo {
    pub fn new(field_unit_health_point_map: HashMap<i32, i32>) -> Self {
        FieldUnitHealthPointInfo {
            field_unit_health_point_map
        }
    }
}