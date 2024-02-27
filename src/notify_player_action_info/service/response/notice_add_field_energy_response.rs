use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_field_energy_info::PlayerFieldEnergyInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeAddFieldEnergyResponse {
    player_field_energy_info: PlayerFieldEnergyInfo,
}

impl NoticeAddFieldEnergyResponse {
    pub fn new(player_field_energy_info: PlayerFieldEnergyInfo) -> Self {
        NoticeAddFieldEnergyResponse {
            player_field_energy_info
        }
    }

    pub fn get_player_field_energy_info(&self) -> &PlayerFieldEnergyInfo {
        &self.player_field_energy_info
    }
}