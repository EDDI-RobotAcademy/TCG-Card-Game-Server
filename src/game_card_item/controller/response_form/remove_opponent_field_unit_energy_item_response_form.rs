use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_death_data_response::GenerateOpponentSpecificUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_energy_data_response::GenerateOpponentSpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_health_point_data_response::GenerateOpponentSpecificUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveOpponentFieldUnitEnergyItemResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl RemoveOpponentFieldUnitEnergyItemResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> Self {
        RemoveOpponentFieldUnitEnergyItemResponseForm {
            is_success,
            false_message_enum,
            player_field_unit_energy_map,
            player_field_unit_health_point_map,
            player_field_unit_death_map,
        }
    }

    pub fn from_response(
        generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
        generate_opponent_specific_unit_energy_data_response: GenerateOpponentSpecificUnitEnergyDataResponse,
    ) -> RemoveOpponentFieldUnitEnergyItemResponseForm {

        RemoveOpponentFieldUnitEnergyItemResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_opponent_specific_unit_energy_data_response.get_player_field_unit_energy_map_for_response().clone(),
            HashMap::new(),
            HashMap::new())
    }

    pub fn from_alternative_response(
        generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
        generate_opponent_specific_unit_health_point_data_response: GenerateOpponentSpecificUnitHealthPointDataResponse,
        generate_opponent_specific_unit_death_data_response: GenerateOpponentSpecificUnitDeathDataResponse
    ) -> RemoveOpponentFieldUnitEnergyItemResponseForm {

        RemoveOpponentFieldUnitEnergyItemResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            HashMap::new(),
            generate_opponent_specific_unit_health_point_data_response.get_player_field_unit_health_point_map_for_response().clone(),
            generate_opponent_specific_unit_death_data_response.get_player_field_unit_death_map_for_response().clone())
    }

    pub fn default() -> RemoveOpponentFieldUnitEnergyItemResponseForm {
        RemoveOpponentFieldUnitEnergyItemResponseForm::new(
            false,
            -1,
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }

    pub fn from_false_response_with_message(
        false_message: FalseMessage) -> RemoveOpponentFieldUnitEnergyItemResponseForm {
        RemoveOpponentFieldUnitEnergyItemResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }
}