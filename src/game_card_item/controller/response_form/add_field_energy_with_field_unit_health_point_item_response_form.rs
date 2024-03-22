use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

use crate::ui_data_generator::service::response::generate_my_field_energy_data_response::GenerateMyFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_death_data_response::GenerateMySpecificUnitDeathDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFieldEnergyWithFieldUnitHealthPointResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_energy_map: HashMap<PlayerIndex, i32>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl AddFieldEnergyWithFieldUnitHealthPointResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_energy_map: HashMap<PlayerIndex, i32>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,) -> Self {
        AddFieldEnergyWithFieldUnitHealthPointResponseForm {
            is_success,
            false_message_enum,
            player_field_energy_map,
            player_field_unit_death_map
        }
    }

    pub fn from_response(
        generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
        generate_my_field_energy_data_response: GenerateMyFieldEnergyDataResponse,
        generate_my_specific_unit_death_data_response: GenerateMySpecificUnitDeathDataResponse
    ) -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_my_field_energy_data_response
                .get_player_field_energy_map_for_response().clone(),
            generate_my_specific_unit_death_data_response
                .get_player_field_unit_death_map_for_response().clone())
    }

    pub fn default() -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(
            false,
            -1,
            HashMap::new(),
            HashMap::new())
    }

    pub fn from_false_response_with_message(
        false_message: FalseMessage) -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new()
        )
    }
}