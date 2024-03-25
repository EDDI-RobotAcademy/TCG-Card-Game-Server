use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_opponent_field_energy_data_response::GenerateOpponentFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveOpponentFieldEnergyItemResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_energy_map: HashMap<PlayerIndex, i32>
}

impl RemoveOpponentFieldEnergyItemResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_energy_map: HashMap<PlayerIndex, i32>) -> Self {
        RemoveOpponentFieldEnergyItemResponseForm {
            is_success,
            false_message_enum,
            player_field_energy_map
        }
    }

    pub fn from_response(generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_opponent_field_energy_data_response: GenerateOpponentFieldEnergyDataResponse)
        -> RemoveOpponentFieldEnergyItemResponseForm {

        RemoveOpponentFieldEnergyItemResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_opponent_field_energy_data_response
                .get_player_field_energy_map_for_response().clone())
    }

    pub fn default() -> RemoveOpponentFieldEnergyItemResponseForm {

        RemoveOpponentFieldEnergyItemResponseForm::new(
            false,
            -1,
            HashMap::new())
    }

    pub fn from_false_response_with_message(false_message: FalseMessage) -> RemoveOpponentFieldEnergyItemResponseForm {
        RemoveOpponentFieldEnergyItemResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
        )
    }
}