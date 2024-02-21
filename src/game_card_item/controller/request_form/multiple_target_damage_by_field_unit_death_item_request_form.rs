use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_field_unit::service::request::apply_damage_to_target_unit_index_request::ApplyDamageToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::apply_instant_death_to_target_unit_index_request::ApplyInstantDeathToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_field_unit_by_index_request::GetCurrentHealthPointOfFieldUnitByIndexRequest;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_item_card_request::IsItItemCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct MultipleTargetDamageByFieldUnitDeathItemRequestForm {
    unit_index: String,
    session_id: String,
    item_card_id: String,
    opponent_target_unit_index_list: Vec<String>,
}

impl MultipleTargetDamageByFieldUnitDeathItemRequestForm {
    pub fn new(unit_index: &str, session_id: &str, item_card_id: &str, opponent_target_unit_index_list: Vec<String>,) -> Self {
        MultipleTargetDamageByFieldUnitDeathItemRequestForm {
            unit_index: unit_index.to_string(),
            session_id: session_id.to_string(),
            item_card_id: item_card_id.to_string(),
            opponent_target_unit_index_list
        }
    }

    pub fn get_unit_index(&self) -> &str { &self.unit_index }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_item_card_id(&self) -> &str {
        &self.item_card_id
    }

    pub fn get_opponent_target_unit_index_list(&self) -> &Vec<String> { &self.opponent_target_unit_index_list }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }

    pub fn to_check_protocol_hacking_request(&self,
                                             account_unique_id: i32,
                                             item_card_id: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id,
                                         item_card_id)
    }

    pub fn to_is_it_item_card_request(&self,
                                      item_card_id: i32) -> IsItItemCardRequest {
        IsItItemCardRequest::new(item_card_id)
    }

    pub fn to_can_use_card_request(&self,
                                   account_unique_id: i32,
                                   item_card_id: i32) -> CanUseCardRequest {
        CanUseCardRequest::new(account_unique_id,
                               item_card_id)
    }

    pub fn to_summary_item_effect_request(&self,
                                          item_card_id: i32) -> SummaryItemCardEffectRequest {
        SummaryItemCardEffectRequest::new(item_card_id)
    }

    pub fn to_find_opponent_by_account_id_request(&self,
                                                  account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(account_unique_id)
    }

    pub fn to_use_game_hand_item_card_request(&self,
                                              account_unique_id: i32,
                                              item_card_id: i32) -> UseGameHandItemCardRequest {
        UseGameHandItemCardRequest::new(account_unique_id,
                                        item_card_id)
    }

    pub fn to_place_to_tomb_request(&self,
                                    account_unique_id: i32,
                                    used_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(account_unique_id,
                                used_card_id)
    }

    pub fn to_find_target_unit_id_by_index_request(&self,
                                                   account_unique_id: i32,
                                                   field_unit_index: i32) -> FindTargetUnitIdByIndexRequest {
        FindTargetUnitIdByIndexRequest::new(account_unique_id,
                                            field_unit_index)
    }

    pub fn to_get_current_health_point_of_field_unit_by_index_request(&self,
                                                                      account_unique_id: i32,
                                                                      field_unit_index: i32) -> GetCurrentHealthPointOfFieldUnitByIndexRequest {
        GetCurrentHealthPointOfFieldUnitByIndexRequest::new(account_unique_id,
                                                            field_unit_index)
    }

    pub fn to_apply_instance_death_to_field_unit_request(&self,
                                           account_unique_id: i32,
                                           field_unit_index: i32) -> ApplyInstantDeathToTargetUnitIndexRequest {
        ApplyInstantDeathToTargetUnitIndexRequest::new(account_unique_id,
                                                       field_unit_index)
    }

    pub fn to_apply_damage_to_target_unit_request(&self,
                                                  opponent_unique_id: i32,
                                                  opponent_target_unit_index: i32,
                                                  damage: i32) -> ApplyDamageToTargetUnitIndexRequest {
        ApplyDamageToTargetUnitIndexRequest::new(opponent_unique_id,
                                                 opponent_target_unit_index,
                                                 damage)
    }
}