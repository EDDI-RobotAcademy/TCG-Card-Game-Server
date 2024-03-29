use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMySpecificUnitEnergyDataResponse {
    player_field_unit_energy_map_for_response: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
    player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl GenerateMySpecificUnitEnergyDataResponse {
    pub fn new(player_field_unit_energy_map_for_response: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
               player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,) -> Self {

        GenerateMySpecificUnitEnergyDataResponse {
            player_field_unit_energy_map_for_response,
            player_field_unit_energy_map_for_notice,
        }
    }

    pub fn get_player_field_unit_energy_map_for_response(&self) -> &HashMap<PlayerIndex, FieldUnitEnergyInfo> {
        &self.player_field_unit_energy_map_for_response
    }

    pub fn get_player_field_unit_energy_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitEnergyInfo> {
        &self.player_field_unit_energy_map_for_notice
    }
}