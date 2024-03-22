use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

use crate::ui_data_generator::service::response::generate_opponent_deck_card_lost_data_response::GenerateOpponentDeckCardLostDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_main_character_health_point_data_response::GenerateOpponentMainCharacterHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_main_character_survival_data_response::GenerateOpponentMainCharacterSurvivalDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_death_data_response::GenerateOpponentMultipleUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_health_point_data_response::GenerateOpponentMultipleUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatastrophicDamageItemResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
    player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    player_deck_card_lost_list_map: HashMap<PlayerIndex, Vec<i32>>,
}

impl CatastrophicDamageItemResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
               player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
               player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
               player_deck_card_lost_list_map: HashMap<PlayerIndex, Vec<i32>>
    ) -> Self {
        CatastrophicDamageItemResponseForm {
            is_success,
            false_message_enum,
            player_field_unit_health_point_map,
            player_field_unit_death_map,
            player_main_character_health_point_map,
            player_main_character_survival_map,
            player_deck_card_lost_list_map
        }
    }

    pub fn from_response(
        generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
        generate_opponent_multiple_unit_health_point_data_response: GenerateOpponentMultipleUnitHealthPointDataResponse,
        generate_opponent_multiple_unit_death_data_response: GenerateOpponentMultipleUnitDeathDataResponse,
        generate_opponent_main_character_health_point_data_response: GenerateOpponentMainCharacterHealthPointDataResponse,
        generate_opponent_main_character_survival_data_response: GenerateOpponentMainCharacterSurvivalDataResponse,
        generate_opponent_deck_card_lost_data_response: GenerateOpponentDeckCardLostDataResponse,
    ) -> CatastrophicDamageItemResponseForm {

        CatastrophicDamageItemResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_opponent_multiple_unit_health_point_data_response.get_player_field_unit_health_point_map_for_response().clone(),
            generate_opponent_multiple_unit_death_data_response.get_player_field_unit_death_map_for_response().clone(),
            generate_opponent_main_character_health_point_data_response.get_player_main_character_health_point_map_for_response().clone(),
            generate_opponent_main_character_survival_data_response.get_player_main_character_survival_map_for_response().clone(),
            generate_opponent_deck_card_lost_data_response.get_player_deck_card_lost_list_map_for_response().clone()
        )
    }

    pub fn default() -> CatastrophicDamageItemResponseForm {
        CatastrophicDamageItemResponseForm::new(
            false,
            -1,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }

    pub fn from_false_response_with_message(
        false_message: FalseMessage) -> CatastrophicDamageItemResponseForm {

        CatastrophicDamageItemResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }
}