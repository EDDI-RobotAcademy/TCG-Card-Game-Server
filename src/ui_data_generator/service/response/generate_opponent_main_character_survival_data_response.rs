use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOpponentMainCharacterSurvivalDataResponse {
    player_main_character_survival_map_for_response: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>
}

impl GenerateOpponentMainCharacterSurvivalDataResponse {
    pub fn new(player_main_character_survival_map_for_response: HashMap<PlayerIndex, StatusMainCharacterEnum>,
               player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> Self {
        GenerateOpponentMainCharacterSurvivalDataResponse {
            player_main_character_survival_map_for_response,
            player_main_character_survival_map_for_notice,
        }
    }

    pub fn get_player_main_character_survival_map_for_response(&self) -> &HashMap<PlayerIndex, StatusMainCharacterEnum> {
        &self.player_main_character_survival_map_for_response
    }

    pub fn get_player_main_character_survival_map_for_notice(&self) -> &HashMap<PlayerIndex, StatusMainCharacterEnum> {
        &self.player_main_character_survival_map_for_notice
    }
}