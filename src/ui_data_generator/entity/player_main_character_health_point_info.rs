use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerMainCharacterHealthPointInfo {
    player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
}

impl PlayerMainCharacterHealthPointInfo {
    pub fn new(player_main_character_health_point_map: HashMap<PlayerIndex, i32>) -> Self {
        PlayerMainCharacterHealthPointInfo {
            player_main_character_health_point_map
        }
    }
    pub fn get_player_main_character_health_point_map(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_main_character_health_point_map
    }
}