use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::harmful_effect_info::HarmfulStatusInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitHarmfulStatusInfo {
    field_unit_harmful_status_map: HashMap<i32, HarmfulStatusInfo>,
}

impl FieldUnitHarmfulStatusInfo {
    pub fn new(field_unit_harmful_status_map: HashMap<i32, HarmfulStatusInfo>) -> Self {
        FieldUnitHarmfulStatusInfo {
            field_unit_harmful_status_map
        }
    }
}