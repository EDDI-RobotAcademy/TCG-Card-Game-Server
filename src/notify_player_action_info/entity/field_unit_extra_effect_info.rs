use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::extra_effect_info::ExtraEffectInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitExtraEffectInfo {
    field_unit_extra_effect_map: HashMap<i32, ExtraEffectInfo>,
}

impl FieldUnitExtraEffectInfo {
    pub fn new(field_unit_extra_effect_map: HashMap<i32, ExtraEffectInfo>) -> Self {
        FieldUnitExtraEffectInfo {
            field_unit_extra_effect_map
        }
    }
}