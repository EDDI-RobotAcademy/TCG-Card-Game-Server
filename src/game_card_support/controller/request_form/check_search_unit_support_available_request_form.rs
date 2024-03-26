use crate::game_card_support::service::request::summarize_support_card_effect_request::{SummarizeSupportCardEffectRequest};
use crate::game_card_support_usage_counter::service::request::check_support_card_usage_count_request::CheckSupportCardUsageCountRequest;
use crate::game_deck::service::request::find_deck_card_id_by_index_request::FindDeckCardIdByIndexRequest;
use crate::game_deck::service::request::game_deck_card_list_request::GameDeckCardListRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_protocol_validation::service::request::is_it_unit_card_request::IsItUnitCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct CheckSearchUnitSupportAvailableRequestForm {
    session_id: String,
    support_card_number: String,
}

impl CheckSearchUnitSupportAvailableRequestForm {
    pub fn new(session_id: &str, support_card_number: &str) -> Self {
        CheckSearchUnitSupportAvailableRequestForm {
            session_id: session_id.to_string(),
            support_card_number: support_card_number.to_string(),
        }
    }

    pub fn get_support_card_number(&self) -> &str { &self.support_card_number }

    pub fn to_session_validation_request(
        &self) -> GetValueWithKeyRequest {

        GetValueWithKeyRequest::new(
            self.session_id.clone().as_str())
    }

    pub fn to_is_this_your_turn_request(
        &self,
        account_unique_id: i32) -> IsThisYourTurnRequest {

        IsThisYourTurnRequest::new(
            account_unique_id)
    }

    pub fn to_check_protocol_hacking_request(
        &self,
        account_unique_id: i32,
        support_card_number: i32) -> CheckProtocolHackingRequest {

        CheckProtocolHackingRequest::new(
            account_unique_id,
            support_card_number)
    }

    pub fn to_can_use_card_request(
        &self,
        account_unique_id: i32,
        support_card_number: i32) -> CanUseCardRequest {

        CanUseCardRequest::new(
            account_unique_id,
            support_card_number)
    }

    pub fn to_is_it_support_card_request(
        &self,
        support_card_number: i32) -> IsItSupportCardRequest {

        IsItSupportCardRequest::new(
            support_card_number)
    }

    pub fn to_find_deck_card_id_by_index_request(
        &self,
        account_unique_id: i32,
        target_card_index: i32) -> FindDeckCardIdByIndexRequest {

        FindDeckCardIdByIndexRequest::new(
            account_unique_id,
            target_card_index)
    }

    pub fn to_is_it_unit_card_request(
        &self,
        unit_card_number: i32) -> IsItUnitCardRequest {

        IsItUnitCardRequest::new(
            unit_card_number)
    }

    pub fn to_check_support_card_usage_count_request(
        &self,
        account_unique_id: i32) -> CheckSupportCardUsageCountRequest {

        CheckSupportCardUsageCountRequest::new(
            account_unique_id)
    }

    pub fn to_summarize_support_card_effect_request(
        &self,
        support_card_number: i32) -> SummarizeSupportCardEffectRequest {

        SummarizeSupportCardEffectRequest::new(
            support_card_number)
    }

    pub fn to_get_deck_request(
        &self) -> GameDeckCardListRequest {

        GameDeckCardListRequest::new(
            self.session_id.clone())
    }
}