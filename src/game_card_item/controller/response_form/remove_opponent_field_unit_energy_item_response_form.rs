use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::notify_player_action_info::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::notify_player_action_info::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_specific_opponent_unit_response::NoticeApplyDamageToSpecificOpponentUnitResponse;
use crate::notify_player_action_info::service::response::notice_remove_energy_of_specific_opponent_unit_response::NoticeRemoveEnergyOfSpecificOpponentUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveOpponentFieldUnitEnergyItemResponseForm {
    is_success: bool,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
    player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl RemoveOpponentFieldUnitEnergyItemResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
               player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> Self {
        RemoveOpponentFieldUnitEnergyItemResponseForm {
            is_success,
            player_field_unit_energy_map,
            player_field_unit_damage_map,
            player_field_unit_health_point_map,
            player_field_unit_death_map,
        }
    }

    pub fn from_response(
        notice_use_hand_card_response: NoticeUseHandCardResponse,
        notice_remove_energy_of_specific_opponent_unit_response: NoticeRemoveEnergyOfSpecificOpponentUnitResponse
    ) -> RemoveOpponentFieldUnitEnergyItemResponseForm {

        RemoveOpponentFieldUnitEnergyItemResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_remove_energy_of_specific_opponent_unit_response
                .get_player_field_unit_energy_info()
                .get_player_field_unit_energy_map().clone(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }

    pub fn from_alternative_response(
        notice_use_hand_card_response: NoticeUseHandCardResponse,
        notice_apply_damage_to_specific_opponent_unit_response: NoticeApplyDamageToSpecificOpponentUnitResponse
    ) -> RemoveOpponentFieldUnitEnergyItemResponseForm {

        RemoveOpponentFieldUnitEnergyItemResponseForm::new(
            notice_use_hand_card_response.is_success(),
            HashMap::new(),
            notice_apply_damage_to_specific_opponent_unit_response
                .get_player_field_unit_damage_info()
                .get_player_field_unit_damage_map().clone(),
            notice_apply_damage_to_specific_opponent_unit_response
                .get_player_field_unit_health_point_info()
                .get_player_field_unit_health_point_map().clone(),
            notice_apply_damage_to_specific_opponent_unit_response
                .get_player_field_unit_death_info()
                .get_player_field_unit_death_map().clone())
    }

    pub fn default() -> RemoveOpponentFieldUnitEnergyItemResponseForm {
        RemoveOpponentFieldUnitEnergyItemResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }
}