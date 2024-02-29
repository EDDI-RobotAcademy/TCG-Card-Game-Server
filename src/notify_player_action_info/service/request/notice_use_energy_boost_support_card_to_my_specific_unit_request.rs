use std::collections::HashMap;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug)]
pub struct NoticeUseEnergyBoostSupportCardToSpecificUnitRequest {
    opponent_unique_id: i32,
    player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl NoticeUseEnergyBoostSupportCardToSpecificUnitRequest {
    pub fn new(opponent_unique_id: i32,
               player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
               player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,) -> Self {
        NoticeUseEnergyBoostSupportCardToSpecificUnitRequest {
            opponent_unique_id,
            player_hand_use_map_for_notice,
            player_deck_card_use_list_map_for_notice,
            player_field_unit_energy_map,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_player_hand_use_map_for_notice(&self) -> &HashMap<PlayerIndex, UsedHandCardInfo> {
        &self.player_hand_use_map_for_notice
    }

    pub fn get_player_deck_card_use_list_map_for_notice(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_deck_card_use_list_map_for_notice
    }

    pub fn get_player_field_unit_energy_map(&self) -> &HashMap<PlayerIndex, FieldUnitEnergyInfo> {
        &self.player_field_unit_energy_map
    }
}