use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::attached_energy_info::AttachedEnergyInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitEnergyInfo {
    field_unit_energy_map: HashMap<i32, AttachedEnergyInfo>,
}

impl FieldUnitEnergyInfo {
    pub fn new(field_unit_energy_map: HashMap<i32, AttachedEnergyInfo>) -> Self {
        FieldUnitEnergyInfo {
            field_unit_energy_map
        }
    }
}