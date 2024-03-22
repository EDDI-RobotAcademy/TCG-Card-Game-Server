use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::game_card_support::controller::response_form::search_unit_support_response_form::SearchUnitSupportResponseForm;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_opponent_field_energy_data_response::GenerateOpponentFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveOpponentFieldEnergySupportResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_energy_map: HashMap<PlayerIndex, i32>
}

impl RemoveOpponentFieldEnergySupportResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_energy_map: HashMap<PlayerIndex, i32>) -> Self {
        RemoveOpponentFieldEnergySupportResponseForm {
            is_success,
            false_message_enum,
            player_field_energy_map
        }
    }

    pub fn from_response(generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_opponent_field_energy_data_response: GenerateOpponentFieldEnergyDataResponse)
        -> RemoveOpponentFieldEnergySupportResponseForm {

        RemoveOpponentFieldEnergySupportResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_opponent_field_energy_data_response
                .get_player_field_energy_map_for_response().clone())
    }

    pub fn default() -> RemoveOpponentFieldEnergySupportResponseForm {

        RemoveOpponentFieldEnergySupportResponseForm::new(
            false,
            -1,
            HashMap::new())
    }

    pub fn from_false_response_with_message(false_message: FalseMessage) -> RemoveOpponentFieldEnergySupportResponseForm {
        RemoveOpponentFieldEnergySupportResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
        )
    }
}