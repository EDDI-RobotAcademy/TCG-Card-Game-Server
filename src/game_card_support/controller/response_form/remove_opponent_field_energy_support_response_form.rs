use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::service::response::notice_remove_field_energy_of_opponent_response::NoticeRemoveFieldEnergyOfOpponentResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;
use crate::ui_data_generator::service::response::generate_opponent_field_energy_data_response::GenerateOpponentFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveOpponentFieldEnergySupportResponseForm {
    is_success: bool,
    player_field_energy_map: HashMap<PlayerIndex, i32>
}

impl RemoveOpponentFieldEnergySupportResponseForm {
    pub fn new(is_success: bool,
               player_field_energy_map: HashMap<PlayerIndex, i32>) -> Self {
        RemoveOpponentFieldEnergySupportResponseForm {
            is_success,
            player_field_energy_map
        }
    }

    pub fn from_response(generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_opponent_field_energy_data_response: GenerateOpponentFieldEnergyDataResponse)
        -> RemoveOpponentFieldEnergySupportResponseForm {

        RemoveOpponentFieldEnergySupportResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            generate_opponent_field_energy_data_response
                .get_player_field_energy_map_for_response().clone())
    }

    pub fn default() -> RemoveOpponentFieldEnergySupportResponseForm {

        RemoveOpponentFieldEnergySupportResponseForm::new(
            false,
            HashMap::new())
    }
}