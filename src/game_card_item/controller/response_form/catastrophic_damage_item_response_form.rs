use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::notify_player_action_info::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::notify_player_action_info::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::notify_player_action_info::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::entity::player_main_character_damage_info::PlayerMainCharacterDamageInfo;
use crate::notify_player_action_info::entity::player_main_character_health_point_info::PlayerMainCharacterHealthPointInfo;
use crate::notify_player_action_info::entity::player_main_character_survival_info::PlayerMainCharacterSurvivalInfo;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_every_opponent_unit_response::NoticeApplyDamageToEveryOpponentUnitResponse;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_opponent_main_character_response::NoticeApplyDamageToOpponentMainCharacterResponse;
use crate::notify_player_action_info::service::response::notice_lost_deck_card_opponent_response::NoticeLostDeckCardOfOpponentResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatastrophicDamageItemResponseForm {
    is_success: bool,
    player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    player_main_character_damage_map: HashMap<PlayerIndex, i32>,
    player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
    player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    player_deck_card_lost_list_map: HashMap<PlayerIndex, Vec<i32>>,
}

impl CatastrophicDamageItemResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_damage_map: HashMap<PlayerIndex, FieldUnitDamageInfo>,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
               player_main_character_damage_map: HashMap<PlayerIndex, i32>,
               player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
               player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
               player_deck_card_lost_list_map: HashMap<PlayerIndex, Vec<i32>>
    ) -> Self {
        CatastrophicDamageItemResponseForm {
            is_success,
            player_field_unit_damage_map,
            player_field_unit_health_point_map,
            player_field_unit_death_map,
            player_main_character_damage_map,
            player_main_character_health_point_map,
            player_main_character_survival_map,
            player_deck_card_lost_list_map
        }
    }

    pub fn from_response(
        notice_use_hand_card_response: NoticeUseHandCardResponse,
        notice_apply_damage_to_every_opponent_unit_response: NoticeApplyDamageToEveryOpponentUnitResponse,
        notice_apply_damage_to_opponent_main_character_response: NoticeApplyDamageToOpponentMainCharacterResponse,
        notice_lost_deck_card_of_opponent_response: NoticeLostDeckCardOfOpponentResponse
    ) -> CatastrophicDamageItemResponseForm {

        CatastrophicDamageItemResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_apply_damage_to_every_opponent_unit_response
                .get_player_field_unit_damage_info()
                .get_player_field_unit_damage_map().clone(),
            notice_apply_damage_to_every_opponent_unit_response
                .get_player_field_unit_health_point_info()
                .get_player_field_unit_health_point_map().clone(),
            notice_apply_damage_to_every_opponent_unit_response
                .get_player_field_unit_death_info()
                .get_player_field_unit_death_map().clone(),
            notice_apply_damage_to_opponent_main_character_response
                .get_player_main_character_damage_info()
                .get_player_main_character_damage_map().clone(),
            notice_apply_damage_to_opponent_main_character_response
                .get_player_main_character_health_point_info()
                .get_player_main_character_health_point_map().clone(),
            notice_apply_damage_to_opponent_main_character_response
                .get_player_main_character_survival_info()
                .get_player_main_character_survival_map().clone(),
            notice_lost_deck_card_of_opponent_response
                .get_player_deck_card_lost_list_info()
                .get_player_deck_card_lost_list_map().clone()
        )
    }

    pub fn default() -> CatastrophicDamageItemResponseForm {
        CatastrophicDamageItemResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }
}