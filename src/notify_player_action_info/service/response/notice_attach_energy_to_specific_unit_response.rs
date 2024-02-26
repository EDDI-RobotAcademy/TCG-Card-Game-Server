use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeAttachEnergyToSpecificUnitResponse {
    player_field_unit_energy_info: PlayerFieldUnitEnergyInfo,
}

impl NoticeAttachEnergyToSpecificUnitResponse {
    pub fn new(player_field_unit_energy_info: PlayerFieldUnitEnergyInfo,) -> Self {
        NoticeAttachEnergyToSpecificUnitResponse {
            player_field_unit_energy_info
        }
    }

    pub fn get_player_field_unit_energy_info(&self) -> &PlayerFieldUnitEnergyInfo {
        &self.player_field_unit_energy_info
    }
}