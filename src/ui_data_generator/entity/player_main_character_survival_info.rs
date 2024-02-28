use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerMainCharacterSurvivalInfo {
    player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
}

impl PlayerMainCharacterSurvivalInfo {
    pub fn new(player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>) -> Self {
        PlayerMainCharacterSurvivalInfo {
            player_main_character_survival_map
        }
    }
    pub fn get_player_main_character_survival_map(&self) -> &HashMap<PlayerIndex, StatusMainCharacterEnum> {
        &self.player_main_character_survival_map
    }
}