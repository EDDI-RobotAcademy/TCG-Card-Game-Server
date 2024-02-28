use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerMainCharacterDamageInfo {
    player_main_character_damage_map: HashMap<PlayerIndex, i32>,
}

impl PlayerMainCharacterDamageInfo {
    pub fn new(player_main_character_damage_map: HashMap<PlayerIndex, i32>) -> Self {
        PlayerMainCharacterDamageInfo {
            player_main_character_damage_map
        }
    }
    pub fn get_player_main_character_damage_map(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_main_character_damage_map
    }
}