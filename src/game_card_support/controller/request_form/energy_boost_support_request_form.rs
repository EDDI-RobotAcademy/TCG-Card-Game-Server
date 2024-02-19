use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::service::request::summarize_support_card_effect_request::SummarizeSupportCardEffectRequest;
use crate::game_card_support_usage_counter::service::request::check_support_card_usage_count_request::CheckSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::update_support_card_usage_count_request::UpdateSupportCardUsageCountRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_deck::service::request::found_card_from_deck_request::FoundCardFromDeckRequest;
use crate::game_field_unit::service::request::attach_multiple_energy_to_unit_index_request::AttachMultipleEnergyToUnitIndexRequest;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_boost_card_request::NotifyToOpponentYouUseEnergyBoostCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct EnergyBoostSupportRequestForm {
    session_id: String,
    unit_index_number: String,
    support_card_id: String,
}

impl EnergyBoostSupportRequestForm {
    pub fn new(session_id: String, unit_index_number: String, support_card_number: String) -> Self {
        EnergyBoostSupportRequestForm {
            session_id: session_id.to_string(),
            unit_index_number: unit_index_number.to_string(),
            support_card_id: support_card_number.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
    pub fn get_unit_index_number(&self) -> &str {
        &self.unit_index_number
    }
    pub fn get_support_card_id(&self) -> &str {
        &self.support_card_id
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
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
        UseGameHandSupportCardRequest::new(
            account_unique_id, support_card_number)
    }

    pub fn to_place_to_tomb_request(&self, account_unique_id: i32, used_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(account_unique_id, used_card_id)
    }

    pub fn to_summarize_support_card_effect_request(&self, support_card_number: i32) -> SummarizeSupportCardEffectRequest {
        SummarizeSupportCardEffectRequest::new(support_card_number)
    }

    pub fn to_found_card_from_deck_request(&self, account_unique_id: i32, need_to_find_card_id: i32, energy_count: i32) -> FoundCardFromDeckRequest {
        FoundCardFromDeckRequest::new(account_unique_id, need_to_find_card_id, energy_count)
    }

    pub fn to_attach_multiple_energy_to_unit_index_request(&self, account_unique_id: i32,
                                                           unit_number: i32,
                                                           boost_race: RaceEnum,
                                                           energy_count: i32) -> AttachMultipleEnergyToUnitIndexRequest {

        AttachMultipleEnergyToUnitIndexRequest::new(account_unique_id, unit_number, boost_race, energy_count)
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_notify_to_opponent_you_use_energy_card_request(
        &self, opponent_unique_id: i32,
        unit_card_index: i32,
        usage_hand_card_id: i32,
        boosting_energy_count: i32,
        boosting_energy_card_id: i32) -> NotifyToOpponentYouUseEnergyBoostCardRequest {

            NotifyToOpponentYouUseEnergyBoostCardRequest::new(
                opponent_unique_id, unit_card_index, usage_hand_card_id, boosting_energy_count, boosting_energy_card_id)
    }
}