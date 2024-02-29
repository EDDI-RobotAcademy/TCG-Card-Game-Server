use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMyMainCharacterHealthPointDataResponse {
    player_main_character_health_point_map_for_response: HashMap<PlayerIndex, i32>,
    player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
}

impl GenerateMyMainCharacterHealthPointDataResponse {
    pub fn new(player_main_character_health_point_map_for_response: HashMap<PlayerIndex, i32>,
               player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
    ) -> Self {
        GenerateMyMainCharacterHealthPointDataResponse {
            player_main_character_health_point_map_for_response,
            player_main_character_health_point_map_for_notice,
        }
    }

    pub fn get_player_main_character_health_point_map_for_response(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_main_character_health_point_map_for_response
    }

    pub fn get_player_main_character_health_point_map_for_notice(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_main_character_health_point_map_for_notice
    }
}