use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::GenerateMySpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachGeneralEnergyCardResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl AttachGeneralEnergyCardResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> Self {

        AttachGeneralEnergyCardResponseForm {
            is_success,
            false_message_enum,
            player_field_unit_energy_map
        }
    }

    pub fn from_response(generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_my_specific_unit_energy_data_response: GenerateMySpecificUnitEnergyDataResponse
    ) -> AttachGeneralEnergyCardResponseForm {

        AttachGeneralEnergyCardResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_my_specific_unit_energy_data_response
                .get_player_field_unit_energy_map_for_response().clone())
    }

    pub fn default() -> AttachGeneralEnergyCardResponseForm {

        AttachGeneralEnergyCardResponseForm::new(
            false,
            -1,
            HashMap::new())
    }
    pub fn from_response_with_message(false_message: FalseMessage) -> AttachGeneralEnergyCardResponseForm {

        AttachGeneralEnergyCardResponseForm::new(
            false,
            false_message as i32,
            HashMap::new())
    }
}
