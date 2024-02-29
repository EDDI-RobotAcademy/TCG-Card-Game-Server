use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateDrawOpponentDeckDataResponse {
    player_draw_count_map_for_response: HashMap<PlayerIndex, i32>,
    player_drawn_card_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>

}

impl GenerateDrawOpponentDeckDataResponse {
    pub fn new(player_draw_count_map_for_response: HashMap<PlayerIndex, i32>,
               player_drawn_card_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>) -> Self {
        GenerateDrawOpponentDeckDataResponse {
            player_draw_count_map_for_response,
            player_drawn_card_list_map_for_notice
        }
    }

    pub fn get_player_drawn_card_count_map_for_response(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_draw_count_map_for_response
    }

    pub fn get_player_drawn_card_list_map_for_notice(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_drawn_card_list_map_for_notice
    }
}