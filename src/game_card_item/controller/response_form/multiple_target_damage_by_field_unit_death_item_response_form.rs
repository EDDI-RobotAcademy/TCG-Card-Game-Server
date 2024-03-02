use std::collections::HashMap;
use serde::{Deserialize, Serialize};
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
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl MultipleTargetDamageByFieldUnitDeathItemResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> Self {
        MultipleTargetDamageByFieldUnitDeathItemResponseForm {
            is_success,
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
            generate_opponent_multiple_unit_health_point_data_response
                .get_player_field_unit_health_point_map_for_response().clone(),
            unit_death_map_for_response)
    }

    pub fn default() -> MultipleTargetDamageByFieldUnitDeathItemResponseForm {
        MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new())
    }
}