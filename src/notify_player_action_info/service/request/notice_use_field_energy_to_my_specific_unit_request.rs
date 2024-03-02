use std::collections::HashMap;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug)]
pub struct NoticeUseFieldEnergyToMySpecificUnitRequest {
    opponent_unique_id: i32,
    player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
    player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl NoticeUseFieldEnergyToMySpecificUnitRequest {
    pub fn new(opponent_unique_id: i32,
               player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
               player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,) -> Self {
        NoticeUseFieldEnergyToMySpecificUnitRequest {
            opponent_unique_id,
            player_field_energy_map_for_notice,
            player_field_unit_energy_map_for_notice
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_player_field_energy_map_for_notice(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_field_energy_map_for_notice
    }

    pub fn get_player_field_unit_energy_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitEnergyInfo> {
        &self.player_field_unit_energy_map_for_notice
    }
}