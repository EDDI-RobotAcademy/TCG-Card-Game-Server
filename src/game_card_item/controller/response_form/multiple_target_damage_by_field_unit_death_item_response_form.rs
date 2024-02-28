use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_multiple_opponent_unit_response::NoticeApplyDamageToMultipleOpponentUnitResponse;
use crate::notify_player_action_info::service::response::notice_instant_death_of_specific_unit_response::NoticeInstantDeathOfSpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleTargetDamageByFieldUnitDeathItemResponseForm {
    is_success: bool,
    player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl MultipleTargetDamageByFieldUnitDeathItemResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> Self {
        MultipleTargetDamageByFieldUnitDeathItemResponseForm {
            is_success,
            player_field_unit_damage_map,
            player_field_unit_health_point_map,
            player_field_unit_death_map
        }
    }

    pub fn from_response(
        notice_use_hand_card_response: NoticeUseHandCardResponse,
        notice_instant_death_of_specific_unit_response: NoticeInstantDeathOfSpecificUnitResponse,
        notice_apply_damage_to_multiple_opponent_unit_response: NoticeApplyDamageToMultipleOpponentUnitResponse
    ) -> MultipleTargetDamageByFieldUnitDeathItemResponseForm {

        let mut field_unit_death_map =
            notice_instant_death_of_specific_unit_response
                .get_player_field_unit_death_info()
                .get_player_field_unit_death_map().clone();

        let opponent_field_unit_death_map =
            notice_apply_damage_to_multiple_opponent_unit_response
                .get_player_field_unit_death_info()
                .get_player_field_unit_death_map().clone();

        field_unit_death_map.extend(opponent_field_unit_death_map);

        MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_apply_damage_to_multiple_opponent_unit_response
                .get_player_field_unit_damage_info()
                .get_player_field_unit_damage_map().clone(),
            notice_apply_damage_to_multiple_opponent_unit_response
                .get_player_field_unit_health_point_info()
                .get_player_field_unit_health_point_map().clone(),
            field_unit_death_map)
    }

    pub fn default() -> MultipleTargetDamageByFieldUnitDeathItemResponseForm {
        MultipleTargetDamageByFieldUnitDeathItemResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }
}