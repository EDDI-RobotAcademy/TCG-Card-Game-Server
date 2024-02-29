use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::GenerateMySpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_deck_card_list_data_response::GenerateUseMyDeckCardListDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBoostSupportResponseForm {
    is_success: bool,
    player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl EnergyBoostSupportResponseForm {
    pub fn new(is_success: bool,
               player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,) -> Self {
        EnergyBoostSupportResponseForm {
            is_success,
            player_deck_card_use_list_map,
            player_field_unit_energy_map
        }
    }

    pub fn from_response(generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_use_my_deck_card_list_data_response: GenerateUseMyDeckCardListDataResponse,
                         generate_my_specific_unit_energy_data_response: GenerateMySpecificUnitEnergyDataResponse)
        -> EnergyBoostSupportResponseForm {

        EnergyBoostSupportResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            generate_use_my_deck_card_list_data_response.get_player_deck_card_use_list_map_for_response().clone(),
            generate_my_specific_unit_energy_data_response.get_player_field_unit_energy_map_for_response().clone())
    }

    pub fn default() -> EnergyBoostSupportResponseForm {
        EnergyBoostSupportResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new())
    }
}