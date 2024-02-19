use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_support::service::request::summarize_support_card_effect_request::SummarizeSupportCardEffectRequest;
use crate::game_card_support_usage_counter::service::request::check_support_card_usage_count_request::CheckSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::update_support_card_usage_count_request::UpdateSupportCardUsageCountRequest;
use crate::game_field_energy::service::request::remove_field_energy_with_amount_request::RemoveFieldEnergyWithAmountRequest;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_field_energy_remove_support_card_request::NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct RemoveOpponentFieldEnergySupportRequestForm {
    session_id: String,
    support_card_id: String,
}

impl RemoveOpponentFieldEnergySupportRequestForm {
    pub fn new(session_id: &str, support_card_number: &str) -> Self {
        RemoveOpponentFieldEnergySupportRequestForm {
            session_id: session_id.to_string(),
            support_card_id: support_card_number.to_string(),
        }
    }
    pub fn get_support_card_id(&self) -> &str { &self.support_card_id }
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
        UseGameHandSupportCardRequest::new(account_unique_id, support_card_number)
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
    pub fn to_remove_field_energy_with_amount_request(&self, account_unique_id: i32, amount_to_remove: i32) -> RemoveFieldEnergyWithAmountRequest {
        RemoveFieldEnergyWithAmountRequest::new(account_unique_id, amount_to_remove)
    }
    pub fn to_notify_to_opponent_you_use_field_energy_remove_support_card_request(&self, opponent_unique_id: i32, usage_support_card_id: i32, field_energy_removed: i32) -> NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest {
        NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest::new(opponent_unique_id, usage_support_card_id, field_energy_removed)
    }
}