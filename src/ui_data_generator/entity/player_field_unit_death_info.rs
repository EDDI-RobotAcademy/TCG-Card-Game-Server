use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitDeathInfo {
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl PlayerFieldUnitDeathInfo {
    pub fn new(player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>) -> Self {
        PlayerFieldUnitDeathInfo {
            player_field_unit_death_map
        }
    }
    pub fn get_player_field_unit_death_map(&self) -> &HashMap<PlayerIndex, FieldUnitDeathInfo> {
        &self.player_field_unit_death_map
    }
}