use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_my_specific_unit_death_data_response::GenerateMySpecificUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_death_data_response::GenerateOpponentMultipleUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_health_point_data_response::GenerateOpponentMultipleUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleTargetDamageByFieldUnitDeathItemResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl MultipleTargetDamageByFieldUnitDeathItemResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> Self {
        MultipleTargetDamageByFieldUnitDeathItemResponseForm {
            is_success,
            false_message_enum,
            player_field_unit_health_point_map,
            player_field_unit_death_map
        }
    }

    pub fn from_response(
        generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
        generate_opponent_multiple_unit_health_point_data_response: GenerateOpponentMultipleUnitHealthPointDataResponse,
        generate_opponent_multiple_unit_death_data_response: GenerateOpponentMultipleUnitDeathDataResponse,
        generate_my_specific_unit_death_data_response: GenerateMySpecificUnitDeathDataResponse,
    ) -> MultipleTargetDamageByFieldUnitDeathItemResponseForm {

        let mut unit_death_map_for_response = HashMap::new();
        unit_death_map_for_response.extend(generate_opponent_multiple_unit_death_data_response.get_player_field_unit_death_map_for_response().clone());
        unit_death_map_for_response.extend(generate_my_specific_unit_death_data_response.get_player_field_unit_death_map_for_response().clone());

        MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_opponent_multiple_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_response().clone(),
            unit_death_map_for_response)
    }

    pub fn default() -> MultipleTargetDamageByFieldUnitDeathItemResponseForm {
        MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(
            false,
            -1,
            HashMap::new(),
            HashMap::new())
    }

    pub fn from_false_response_with_message(
        false_message: FalseMessage) -> MultipleTargetDamageByFieldUnitDeathItemResponseForm {
        MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new())
    }
}