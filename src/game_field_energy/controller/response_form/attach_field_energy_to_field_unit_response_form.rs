use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_my_field_energy_data_response::GenerateMyFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::GenerateMySpecificUnitEnergyDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachFieldEnergyToFieldUnitResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_energy_map: HashMap<PlayerIndex, i32>,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl AttachFieldEnergyToFieldUnitResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_energy_map: HashMap<PlayerIndex, i32>,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>) -> Self {
        AttachFieldEnergyToFieldUnitResponseForm {
            is_success,
            false_message_enum,
            player_field_energy_map,
            player_field_unit_energy_map,
        }
    }
    pub fn from_response(generate_my_field_energy_data_response: GenerateMyFieldEnergyDataResponse,
                         generate_my_specific_unit_energy_data_response: GenerateMySpecificUnitEnergyDataResponse
    ) -> AttachFieldEnergyToFieldUnitResponseForm {

        AttachFieldEnergyToFieldUnitResponseForm::new(
            true,
            -1,
            generate_my_field_energy_data_response.get_player_field_energy_map_for_response().clone(),
            generate_my_specific_unit_energy_data_response.get_player_field_unit_energy_map_for_response().clone())
    }

    pub fn default() -> AttachFieldEnergyToFieldUnitResponseForm {
        AttachFieldEnergyToFieldUnitResponseForm::new(
            false,
            -1,
            HashMap::new(),
            HashMap::new())
    }
    pub fn from_response_with_message(false_message: FalseMessage) -> AttachFieldEnergyToFieldUnitResponseForm {
        AttachFieldEnergyToFieldUnitResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new())
    }
}