use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::service::response::notice_add_field_energy_response::NoticeAddFieldEnergyResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

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
        notice_use_hand_card_response: NoticeUseHandCardResponse,
        notice_add_field_energy_response: NoticeAddFieldEnergyResponse
    ) -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_add_field_energy_response
                .get_player_field_energy_info()
                .get_player_field_energy_map().clone())
    }

    pub fn default() -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(
            false,
            HashMap::new())
    }
}