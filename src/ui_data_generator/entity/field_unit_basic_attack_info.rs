use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::basic_attack_info::AttackInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitAttackInfo {
    field_unit_attack_map: HashMap<i32, AttackInfo>,
}

impl FieldUnitAttackInfo {
    pub fn new(field_unit_attack_map: HashMap<i32, AttackInfo>,) -> Self {
        FieldUnitAttackInfo {
            field_unit_attack_map
        }
    }
}