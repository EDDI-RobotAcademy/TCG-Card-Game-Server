use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_field_energy_info::PlayerFieldEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseFieldEnergyToSpecificUnitResponse {
    player_field_unit_energy_info: PlayerFieldUnitEnergyInfo,
    player_field_energy_info: PlayerFieldEnergyInfo,
}

impl NoticeUseFieldEnergyToSpecificUnitResponse {
    pub fn new(player_field_unit_energy_info: PlayerFieldUnitEnergyInfo,
               player_field_energy_info: PlayerFieldEnergyInfo,) -> Self {
        NoticeUseFieldEnergyToSpecificUnitResponse {
            player_field_unit_energy_info,
            player_field_energy_info
        }
    }

    pub fn get_player_field_unit_energy_info(&self) -> &PlayerFieldUnitEnergyInfo {
        &self.player_field_unit_energy_info
    }

    pub fn get_player_field_energy_info(&self) -> &PlayerFieldEnergyInfo {
        &self.player_field_energy_info
    }
}