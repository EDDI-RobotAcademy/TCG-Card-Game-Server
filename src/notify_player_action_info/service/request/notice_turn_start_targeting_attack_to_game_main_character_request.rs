use std::collections::HashMap;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::field_unit_basic_attack_info::FieldUnitAttackInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug)]
pub struct NoticeTurnStartTargetingAttackToGameMainCharacterRequest {
    opponent_unique_id: i32,
    player_field_unit_attack_map_for_notice: HashMap<PlayerIndex, FieldUnitAttackInfo>,
    player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
    player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
}

impl NoticeTurnStartTargetingAttackToGameMainCharacterRequest {
    pub fn new(
        opponent_unique_id: i32,
        player_field_unit_attack_map_for_notice: HashMap<PlayerIndex, FieldUnitAttackInfo>,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> Self {

        NoticeTurnStartTargetingAttackToGameMainCharacterRequest {
            opponent_unique_id,
            player_field_unit_attack_map_for_notice,
            player_main_character_health_point_map_for_notice,
            player_main_character_survival_map_for_notice,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_player_field_unit_attack_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitAttackInfo> {
        &self.player_field_unit_attack_map_for_notice
    }
    pub fn get_player_main_character_health_point_map_for_notice(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_main_character_health_point_map_for_notice
    }

    pub fn get_player_main_character_survival_map_for_notice(&self) -> &HashMap<PlayerIndex, StatusMainCharacterEnum> {
        &self.player_main_character_survival_map_for_notice
    }
}