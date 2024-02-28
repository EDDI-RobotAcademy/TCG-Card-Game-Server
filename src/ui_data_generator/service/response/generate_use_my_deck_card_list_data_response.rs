use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateUseMyDeckCardListDataResponse {
    player_deck_card_use_list_map_for_response: HashMap<PlayerIndex, Vec<i32>>,
    player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
}

impl GenerateUseMyDeckCardListDataResponse {
    pub fn new(player_deck_card_use_list_map_for_response: HashMap<PlayerIndex, Vec<i32>>,
               player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>) -> Self {
        GenerateUseMyDeckCardListDataResponse {
            player_deck_card_use_list_map_for_response,
            player_deck_card_use_list_map_for_notice,
        }
    }

    pub fn get_player_deck_card_use_list_map_for_response(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_deck_card_use_list_map_for_response
    }

    pub fn get_player_deck_card_use_list_map_for_notice(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_deck_card_use_list_map_for_notice
    }
}