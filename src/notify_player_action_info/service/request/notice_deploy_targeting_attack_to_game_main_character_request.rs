use std::collections::HashMap;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug)]
pub struct NoticeDeployTargetingAttackToGameMainCharacterRequest {
    opponent_unique_id: i32,
    player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
    player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
}

impl NoticeDeployTargetingAttackToGameMainCharacterRequest {
    pub fn new(
        opponent_unique_id: i32,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> Self {

        NoticeDeployTargetingAttackToGameMainCharacterRequest {
            opponent_unique_id,
            player_main_character_health_point_map_for_notice,
            player_main_character_survival_map_for_notice,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_player_main_character_health_point_map_for_notice(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_main_character_health_point_map_for_notice
    }

    pub fn get_player_main_character_survival_map_for_notice(&self) -> &HashMap<PlayerIndex, StatusMainCharacterEnum> {
        &self.player_main_character_survival_map_for_notice
    }
}