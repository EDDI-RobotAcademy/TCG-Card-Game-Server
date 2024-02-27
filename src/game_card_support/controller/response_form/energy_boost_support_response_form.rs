use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::attached_energy_info::AttachedEnergyInfo;
use crate::notify_player_action_info::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex::You;
use crate::notify_player_action_info::service::response::notice_boost_energy_to_specific_unit_response::NoticeBoostEnergyToSpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

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

    pub fn from_response(notice_use_hand_card_response: NoticeUseHandCardResponse,
                         notice_boost_energy_to_specific_unit_response: NoticeBoostEnergyToSpecificUnitResponse)
        -> EnergyBoostSupportResponseForm {

        EnergyBoostSupportResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_boost_energy_to_specific_unit_response
                .get_player_deck_card_use_list_info()
                .get_player_deck_card_use_list_map().clone(),
            notice_boost_energy_to_specific_unit_response
                .get_player_field_unit_energy_info()
                .get_player_field_unit_energy_map().clone())
    }

    pub fn default() -> EnergyBoostSupportResponseForm {
        EnergyBoostSupportResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new())
    }
}