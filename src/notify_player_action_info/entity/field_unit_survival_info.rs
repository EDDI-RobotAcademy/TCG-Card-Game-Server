use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitSurvivalInfo {
    field_unit_survival_map: HashMap<i32, bool>,
}

impl FieldUnitSurvivalInfo {
    pub fn new(field_unit_survival_map: HashMap<i32, bool>) -> Self {
        FieldUnitSurvivalInfo {
            field_unit_survival_map
        }
    }
}