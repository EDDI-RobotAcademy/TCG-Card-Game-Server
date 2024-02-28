use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateUseSupportCardToBoostEnergyToMySpecificUnitResponse {
    player_deck_card_use_list_map_for_response: HashMap<PlayerIndex, Vec<i32>>,
    player_field_unit_energy_map_for_response: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
    player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
    player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl GenerateUseSupportCardToBoostEnergyToMySpecificUnitResponse {
    pub fn new(player_deck_card_use_list_map_for_response: HashMap<PlayerIndex, Vec<i32>>,
               player_field_unit_energy_map_for_response: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
               player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
               player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
               player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,) -> Self {
        GenerateUseSupportCardToBoostEnergyToMySpecificUnitResponse {
            player_deck_card_use_list_map_for_response,
            player_field_unit_energy_map_for_response,
            player_hand_use_map_for_notice,
            player_deck_card_use_list_map_for_notice,
            player_field_unit_energy_map_for_notice,
        }
    }

    pub fn get_player_deck_card_use_list_map_for_response(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_deck_card_use_list_map_for_response
    }

    pub fn get_player_field_unit_energy_map_for_response(&self) -> &HashMap<PlayerIndex, FieldUnitEnergyInfo> {
        &self.player_field_unit_energy_map_for_response
    }

    pub fn get_player_hand_use_map_for_notice(&self) -> &HashMap<PlayerIndex, UsedHandCardInfo> {
        &self.player_hand_use_map_for_notice
    }

    pub fn get_player_deck_card_use_list_map_for_notice(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_deck_card_use_list_map_for_notice
    }

    pub fn get_player_field_unit_energy_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitEnergyInfo> {
        &self.player_field_unit_energy_map_for_notice
    }
}