use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerDeckCardUseListInfo {
    player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>,
}

impl PlayerDeckCardUseListInfo {
    pub fn new(player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>) -> Self {
        PlayerDeckCardUseListInfo {
            player_deck_card_use_list_map
        }
    }

    pub fn get_player_deck_card_use_list_map(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_deck_card_use_list_map
    }
}