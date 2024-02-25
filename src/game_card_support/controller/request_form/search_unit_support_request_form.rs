use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_support::service::request::summarize_support_card_effect_request::{SummarizeSupportCardEffectRequest};
use crate::game_card_support_usage_counter::service::request::check_support_card_usage_count_request::CheckSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::update_support_card_usage_count_request::UpdateSupportCardUsageCountRequest;
use crate::game_deck::service::request::game_deck_card_shuffle_request::GameDeckCardShuffleRequest;
use crate::game_deck::service::request::search_specific_deck_card_request::SearchSpecificDeckCardRequest;
use crate::game_hand::service::request::add_card_list_to_hand_request::AddCardListToHandRequest;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_protocol_validation::service::request::is_it_unit_card_request::IsItUnitCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action_info::service::request::notice_search_card_request::{NoticeSearchCardRequest};
use crate::notify_player_action_info::service::request::notice_use_hand_card_request::NoticeUseHandCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct SearchUnitSupportRequestForm {
    session_id: String,
    support_card_number: String,
    target_unit_card_list: Vec<String>,
}

impl SearchUnitSupportRequestForm {
    pub fn new(session_id: &str, support_card_number: &str, target_unit_card_list: Vec<String>) -> Self {
        SearchUnitSupportRequestForm {
            session_id: session_id.to_string(),
            support_card_number: support_card_number.to_string(),
            target_unit_card_list,
        }
    }
    pub fn get_session_id(&self) -> &str { &self.session_id }
    pub fn get_support_card_number(&self) -> &str { &self.support_card_number }
    pub fn get_target_unit_card_list(&self) -> &Vec<String> { &self.target_unit_card_list }
    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }
    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }
    pub fn to_check_protocol_hacking_request(&self, account_unique_id: i32, support_card_number: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id, support_card_number)
    }
    pub fn to_can_use_card_request(&self, account_unique_id: i32, support_card_number: i32) -> CanUseCardRequest {
        CanUseCardRequest::new(account_unique_id, support_card_number)
    }
    pub fn to_is_it_support_card_request(&self, support_card_number: i32) -> IsItSupportCardRequest {
        IsItSupportCardRequest::new(support_card_number)
    }
    pub fn to_check_support_card_usage_count_request(&self, account_unique_id: i32) -> CheckSupportCardUsageCountRequest {
        CheckSupportCardUsageCountRequest::new(account_unique_id)
    }
    pub fn to_update_support_card_usage_count_request(&self, account_unique_id: i32) -> UpdateSupportCardUsageCountRequest {
        UpdateSupportCardUsageCountRequest::new(account_unique_id)
    }
    pub fn to_use_game_hand_support_card_request(&self, account_unique_id: i32, support_card_number: i32) -> UseGameHandSupportCardRequest {
        UseGameHandSupportCardRequest::new(account_unique_id, support_card_number)
    }
    pub fn to_is_it_unit_card_request(&self, unit_card_number: i32) -> IsItUnitCardRequest {
        IsItUnitCardRequest::new(unit_card_number)
    }
    pub fn to_place_to_tomb_request(&self, account_unique_id: i32, used_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(account_unique_id, used_card_id)
    }
    pub fn to_summarize_support_card_effect_request(&self, support_card_number: i32) -> SummarizeSupportCardEffectRequest {
        SummarizeSupportCardEffectRequest::new(support_card_number)
    }
    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(account_unique_id)
    }
    pub fn to_search_specific_deck_card_request(&self, account_unique_id: i32, target_card_id_list: Vec<i32>) -> SearchSpecificDeckCardRequest {
        SearchSpecificDeckCardRequest::new(account_unique_id, target_card_id_list)
    }
    pub fn to_add_card_list_to_hand_request(&self, account_unique_id: i32, card_list: Vec<i32>) -> AddCardListToHandRequest {
        AddCardListToHandRequest::new(account_unique_id, card_list)
    }
    pub fn to_shuffle_deck_request(&self) -> GameDeckCardShuffleRequest {
        GameDeckCardShuffleRequest::new(self.session_id.clone())
    }
    pub fn to_notice_use_hand_card_request(&self,
                                           opponent_unique_id: i32,
                                           used_hand_card_id: i32) -> NoticeUseHandCardRequest {
        NoticeUseHandCardRequest::new(
            opponent_unique_id, used_hand_card_id)
    }
    pub fn to_notice_search_card_request(&self,
                                         opponent_unique_id: i32,
                                         found_card_list: Vec<i32>) -> NoticeSearchCardRequest {
        NoticeSearchCardRequest::new(
            opponent_unique_id, found_card_list)
    }
}