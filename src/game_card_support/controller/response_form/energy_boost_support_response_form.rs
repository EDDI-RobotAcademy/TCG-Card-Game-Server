use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::game_deck::service::response::game_deck_card_list_response::GameDeckCardListResponse;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::GenerateMySpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_deck_card_list_data_response::GenerateUseMyDeckCardListDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBoostSupportResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
    updated_deck_card_list: Vec<i32>,
}

impl EnergyBoostSupportResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
               updated_deck_card_list: Vec<i32>,) -> Self {
        EnergyBoostSupportResponseForm {
            is_success,
            false_message_enum,
            player_deck_card_use_list_map,
            player_field_unit_energy_map,
            updated_deck_card_list
        }
    }

    pub fn from_response(generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_use_my_deck_card_list_data_response: GenerateUseMyDeckCardListDataResponse,
                         generate_my_specific_unit_energy_data_response: GenerateMySpecificUnitEnergyDataResponse,
                         game_deck_card_list_response: GameDeckCardListResponse)
        -> EnergyBoostSupportResponseForm {

        EnergyBoostSupportResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_use_my_deck_card_list_data_response.get_player_deck_card_use_list_map_for_response().clone(),
            generate_my_specific_unit_energy_data_response.get_player_field_unit_energy_map_for_response().clone(),
            game_deck_card_list_response.get_deck_card_list().clone())
    }

    pub fn default() -> EnergyBoostSupportResponseForm {
        EnergyBoostSupportResponseForm::new(
            false,
            -1,
            HashMap::new(),
            HashMap::new(),
            Vec::new())
    }

    pub fn from_false_response_with_message(false_message: FalseMessage) -> EnergyBoostSupportResponseForm {
        EnergyBoostSupportResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new(),
            Vec::new())
    }
}