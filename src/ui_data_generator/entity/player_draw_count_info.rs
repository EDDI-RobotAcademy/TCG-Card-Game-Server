use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerDrawCountInfo {
    player_draw_count_map: HashMap<PlayerIndex, i32>,
}

impl PlayerDrawCountInfo {
    pub fn new(player_draw_count_map: HashMap<PlayerIndex, i32>) -> Self {
        PlayerDrawCountInfo {
            player_draw_count_map
        }
    }

    pub fn get_player_draw_count_map(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_draw_count_map
    }
}