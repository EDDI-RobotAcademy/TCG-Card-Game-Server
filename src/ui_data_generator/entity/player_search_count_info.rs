use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSearchCountInfo {
    player_search_count_map: HashMap<PlayerIndex, i32>,
}

impl PlayerSearchCountInfo {
    pub fn new(player_search_count_map: HashMap<PlayerIndex, i32>) -> Self {
        PlayerSearchCountInfo {
            player_search_count_map
        }
    }
}