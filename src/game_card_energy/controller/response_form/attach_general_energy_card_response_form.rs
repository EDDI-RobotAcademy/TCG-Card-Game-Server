use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::GenerateMySpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachGeneralEnergyCardResponseForm {
    is_success: bool,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl AttachGeneralEnergyCardResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> Self {

        AttachGeneralEnergyCardResponseForm {
            is_success,
            player_field_unit_energy_map
        }
    }

    pub fn from_response(generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_my_specific_unit_energy_data_response: GenerateMySpecificUnitEnergyDataResponse
    ) -> AttachGeneralEnergyCardResponseForm {

        AttachGeneralEnergyCardResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            generate_my_specific_unit_energy_data_response
                .get_player_field_unit_energy_map_for_response().clone())
    }

    pub fn default() -> AttachGeneralEnergyCardResponseForm {

        AttachGeneralEnergyCardResponseForm::new(
            false,
            HashMap::new())
    }
}
