use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_basic_attack_info::FieldUnitAttackInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitAttackInfo {
    player_field_unit_attack_map: HashMap<PlayerIndex, FieldUnitAttackInfo>,
}

impl PlayerFieldUnitAttackInfo {
    pub fn new(player_field_unit_attack_map: HashMap<PlayerIndex, FieldUnitAttackInfo>,) -> Self {
        PlayerFieldUnitAttackInfo {
            player_field_unit_attack_map
        }
    }
    pub fn get_player_field_unit_attack_map(&self) -> &HashMap<PlayerIndex, FieldUnitAttackInfo> {
        &self.player_field_unit_attack_map
    }
}