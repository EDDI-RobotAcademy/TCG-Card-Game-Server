use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitDamageInfo {
    player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
}

impl PlayerFieldUnitDamageInfo {
    pub fn new(player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>) -> Self {
        PlayerFieldUnitDamageInfo {
            player_field_unit_damage_map
        }
    }
    pub fn get_player_field_unit_damage_map(&self) -> &HashMap<PlayerIndex, FieldUnitDamageInfo> {
        &self.player_field_unit_damage_map
    }
}