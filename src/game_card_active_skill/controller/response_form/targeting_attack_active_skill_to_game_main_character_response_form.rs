use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_opponent_main_character_health_point_data_response::GenerateOpponentMainCharacterHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_main_character_survival_data_response::GenerateOpponentMainCharacterSurvivalDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingAttackActiveSkillToGameMainCharacterResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
    player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>

}

impl TargetingAttackActiveSkillToGameMainCharacterResponseForm {
    pub fn new(
        is_success: bool,
        false_message_enum: i32,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>

    ) -> Self {

        TargetingAttackActiveSkillToGameMainCharacterResponseForm {
            is_success,
            false_message_enum,
            player_main_character_health_point_map_for_notice,
            player_main_character_survival_map_for_notice
        }
    }

    pub fn from_response(
        generate_opponent_main_character_health_point_data_response: GenerateOpponentMainCharacterHealthPointDataResponse,
        generate_opponent_main_character_survival_data_response: GenerateOpponentMainCharacterSurvivalDataResponse

    ) -> TargetingAttackActiveSkillToGameMainCharacterResponseForm {

        TargetingAttackActiveSkillToGameMainCharacterResponseForm::new(
            true,
            -1,
            generate_opponent_main_character_health_point_data_response
                .get_player_main_character_health_point_map_for_response().clone(),
            generate_opponent_main_character_survival_data_response
                .get_player_main_character_survival_map_for_response().clone())
    }

    pub fn default() -> TargetingAttackActiveSkillToGameMainCharacterResponseForm {

        TargetingAttackActiveSkillToGameMainCharacterResponseForm::new(false, -1, HashMap::new(), HashMap::new())
    }
    pub fn from_response_with_message(false_message: FalseMessage)
        -> TargetingAttackActiveSkillToGameMainCharacterResponseForm {

        TargetingAttackActiveSkillToGameMainCharacterResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new())
    }
}
