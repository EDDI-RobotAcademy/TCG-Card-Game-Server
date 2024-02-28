use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateUseSupportCardToDrawMyDeckDataResponse {
    player_drawn_card_list_map_for_response: HashMap<PlayerIndex, Vec<i32>>,
    player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_draw_count_map_for_notice: HashMap<PlayerIndex, i32>,
}

impl GenerateUseSupportCardToDrawMyDeckDataResponse {
    pub fn new(player_drawn_card_list_map_for_response: HashMap<PlayerIndex, Vec<i32>>,
               player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
               player_draw_count_map_for_notice: HashMap<PlayerIndex, i32>,) -> Self {
        GenerateUseSupportCardToDrawMyDeckDataResponse {
            player_drawn_card_list_map_for_response,
            player_hand_use_map_for_notice,
            player_draw_count_map_for_notice,
        }
    }

    pub fn get_player_drawn_card_list_map_for_response(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_drawn_card_list_map_for_response
    }

    pub fn get_player_hand_use_map_for_notice(&self) -> &HashMap<PlayerIndex, UsedHandCardInfo> {
        &self.player_hand_use_map_for_notice
    }

    pub fn get_player_draw_count_map_for_notice(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_draw_count_map_for_notice
    }
}