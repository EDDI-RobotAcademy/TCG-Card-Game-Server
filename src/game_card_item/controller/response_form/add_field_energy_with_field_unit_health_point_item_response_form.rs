use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

use crate::ui_data_generator::service::response::generate_my_field_energy_data_response::GenerateMyFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFieldEnergyWithFieldUnitHealthPointResponseForm {
    is_success: bool,
    player_field_energy_map: HashMap<PlayerIndex, i32>
}

impl AddFieldEnergyWithFieldUnitHealthPointResponseForm {
    pub fn new(is_success: bool,
               player_field_energy_map: HashMap<PlayerIndex, i32>) -> Self {
        AddFieldEnergyWithFieldUnitHealthPointResponseForm {
            is_success,
            player_field_energy_map
        }
    }

    pub fn from_response(
        generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
        generate_my_field_energy_data_response: GenerateMyFieldEnergyDataResponse
    ) -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            generate_my_field_energy_data_response
                .get_player_field_energy_map_for_response().clone())
    }

    pub fn default() -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(
            false,
            HashMap::new())
    }
}