use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_deck_card_use_list_info::PlayerDeckCardUseListInfo;
use crate::notify_player_action_info::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeBoostEnergyToSpecificUnitResponse {
    player_deck_card_use_list_info: PlayerDeckCardUseListInfo,
    player_field_unit_energy_info: PlayerFieldUnitEnergyInfo,
}

impl NoticeBoostEnergyToSpecificUnitResponse {
    pub fn new(player_deck_card_use_list_info: PlayerDeckCardUseListInfo,
               player_field_unit_energy_info: PlayerFieldUnitEnergyInfo,) -> Self {
        NoticeBoostEnergyToSpecificUnitResponse {
            player_deck_card_use_list_info,
            player_field_unit_energy_info
        }
    }

    pub fn get_player_deck_card_use_list_info(&self) -> &PlayerDeckCardUseListInfo {
        &self.player_deck_card_use_list_info
    }

    pub fn get_player_field_unit_energy_info(&self) -> &PlayerFieldUnitEnergyInfo {
        &self.player_field_unit_energy_info
    }
}