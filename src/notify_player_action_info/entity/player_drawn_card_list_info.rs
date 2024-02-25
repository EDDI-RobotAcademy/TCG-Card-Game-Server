use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerDrawnCardListInfo {
    player_drawn_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
}

impl PlayerDrawnCardListInfo {
    pub fn new(player_drawn_card_list_map: HashMap<PlayerIndex, Vec<i32>>) -> Self {
        PlayerDrawnCardListInfo {
            player_drawn_card_list_map
        }
    }

    pub fn get_player_drawn_card_list_map(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_drawn_card_list_map
    }
}