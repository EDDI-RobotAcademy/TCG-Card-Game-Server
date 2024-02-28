use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateSearchMyDeckDataResponse {
    player_search_card_list_map_for_response: HashMap<PlayerIndex, Vec<i32>>,
    player_search_count_map_for_notice: HashMap<PlayerIndex, i32>,
}

impl GenerateSearchMyDeckDataResponse {
    pub fn new(player_search_card_list_map_for_response: HashMap<PlayerIndex, Vec<i32>>,
               player_search_count_map_for_notice: HashMap<PlayerIndex, i32>,) -> Self {
        GenerateSearchMyDeckDataResponse {
            player_search_card_list_map_for_response,
            player_search_count_map_for_notice,
        }
    }

    pub fn get_player_search_card_list_map_for_response(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_search_card_list_map_for_response
    }

    pub fn get_player_search_count_map_for_notice(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_search_count_map_for_notice
    }
}