use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_specific_opponent_unit_response::NoticeApplyDamageToSpecificOpponentUnitResponse;
use crate::notify_player_action_info::service::response::notice_instant_death_of_specific_opponent_unit_response::NoticeInstantDeathOfSpecificOpponentUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDeathItemResponseForm {
    is_success: bool,
    player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl TargetDeathItemResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> Self {
        TargetDeathItemResponseForm {
            is_success,
            player_field_unit_damage_map,
            player_field_unit_health_point_map,
            player_field_unit_death_map
        }
    }

    pub fn from_response(
        notice_use_hand_card_response: NoticeUseHandCardResponse,
        notice_instant_death_of_specific_opponent_unit_response: NoticeInstantDeathOfSpecificOpponentUnitResponse
    ) -> TargetDeathItemResponseForm {

        TargetDeathItemResponseForm::new(
            notice_use_hand_card_response.is_success(),
            HashMap::new(),
            HashMap::new(),
            notice_instant_death_of_specific_opponent_unit_response
                .get_player_field_unit_death_info()
                .get_player_field_unit_death_map().clone())
    }

    pub fn from_alternative_response(
        notice_use_hand_card_response: NoticeUseHandCardResponse,
        notice_apply_damage_to_specific_opponent_unit_response: NoticeApplyDamageToSpecificOpponentUnitResponse
    ) -> TargetDeathItemResponseForm {

        TargetDeathItemResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_apply_damage_to_specific_opponent_unit_response
                .get_player_field_unit_damage_info()
                .get_player_field_unit_damage_map().clone(),
            notice_apply_damage_to_specific_opponent_unit_response
                .get_player_field_unit_health_point_info()
                .get_player_field_unit_health_point_map().clone(),
            notice_apply_damage_to_specific_opponent_unit_response
                .get_player_field_unit_death_info()
                .get_player_field_unit_death_map().clone()
        )
    }

    pub fn default() -> TargetDeathItemResponseForm {

        TargetDeathItemResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }
}