use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMyMultipleUnitDeathDataResponse {
    player_field_unit_death_map_for_response: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl GenerateMyMultipleUnitDeathDataResponse {
    pub fn new(player_field_unit_death_map_for_response: HashMap<PlayerIndex, FieldUnitDeathInfo>,
               player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,) -> Self {
        GenerateMyMultipleUnitDeathDataResponse {
            player_field_unit_death_map_for_response,
            player_field_unit_death_map_for_notice,
        }
    }

    pub fn get_player_field_unit_death_map_for_response(&self) -> &HashMap<PlayerIndex, FieldUnitDeathInfo> {
        &self.player_field_unit_death_map_for_response
    }

    pub fn get_player_field_unit_death_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitDeathInfo> {
        &self.player_field_unit_death_map_for_notice
    }
}