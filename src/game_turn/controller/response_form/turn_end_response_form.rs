use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum::Death;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex::{Opponent, You};

use crate::ui_data_generator::service::response::generate_draw_opponent_deck_data_response::GenerateDrawOpponentDeckDataResponse;
use crate::ui_data_generator::service::response::generate_my_multiple_unit_death_data_response::GenerateMyMultipleUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_my_multiple_unit_harmful_effect_data_response::GenerateMyMultipleUnitHarmfulEffectDataResponse;
use crate::ui_data_generator::service::response::generate_my_multiple_unit_health_point_data_response::GenerateMyMultipleUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_field_energy_data_response::GenerateOpponentFieldEnergyDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnEndResponseForm {
    is_success: bool,
    player_draw_count_map: HashMap<PlayerIndex, i32>,
    player_field_energy_map: HashMap<PlayerIndex, i32>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
}

impl TurnEndResponseForm {
    pub fn new(is_success: bool,
               player_draw_count_map: HashMap<PlayerIndex, i32>,
               player_field_energy_map: HashMap<PlayerIndex, i32>,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
               player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> Self {

        TurnEndResponseForm {
            is_success,
            player_draw_count_map,
            player_field_energy_map,
            player_field_unit_health_point_map,
            player_field_unit_harmful_effect_map,
            player_field_unit_death_map,
            player_main_character_survival_map
        }
    }

    pub fn from_response(
        generate_draw_opponent_deck_data_response: GenerateDrawOpponentDeckDataResponse,
        generate_opponent_field_energy_data_response: GenerateOpponentFieldEnergyDataResponse,
        generate_my_multiple_unit_health_point_data_response: GenerateMyMultipleUnitHealthPointDataResponse,
        generate_my_multiple_unit_harmful_effect_data_response: GenerateMyMultipleUnitHarmfulEffectDataResponse,
        generate_my_multiple_unit_death_data_response: GenerateMyMultipleUnitDeathDataResponse
    ) -> TurnEndResponseForm {

        TurnEndResponseForm::new(
            true,
            generate_draw_opponent_deck_data_response
                .get_player_drawn_card_count_map_for_response().clone(),
            generate_opponent_field_energy_data_response
                .get_player_field_energy_map_for_response().clone(),
            generate_my_multiple_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_response().clone(),
            generate_my_multiple_unit_harmful_effect_data_response
                .get_player_field_unit_harmful_effect_map_for_response().clone(),
            generate_my_multiple_unit_death_data_response
                .get_player_field_unit_death_map_for_response().clone(),
            HashMap::new()
        )
    }

    pub fn from_no_more_opponent_deck_response() -> TurnEndResponseForm {

        let mut map_only_for_death_by_no_more_opponent_deck_card = HashMap::new();
        map_only_for_death_by_no_more_opponent_deck_card.insert(Opponent, Death);

        TurnEndResponseForm::new(
            true,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            map_only_for_death_by_no_more_opponent_deck_card
        )
    }

    pub fn default() -> TurnEndResponseForm {

        TurnEndResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }
}
