use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::service::request::summary_energy_card_effect_request::SummaryEnergyCardEffectRequest;
use crate::game_field_unit::service::request::attach_single_energy_to_unit_index_request::AttachSingleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::get_current_attached_energy_of_field_unit_by_index_request::GetCurrentAttachedEnergyOfFieldUnitByIndexRequest;
use crate::game_hand::service::request::use_game_hand_energy_card_request::UseGameHandEnergyCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_energy_card_request::IsItEnergyCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_card_request::NotifyOpponentYouUseEnergyCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct AttachGeneralEnergyCardRequestForm {
    session_id: String,
    unit_card_index: String,
    energy_card_id: String,
}

impl AttachGeneralEnergyCardRequestForm {
    pub fn new(session_id: String, unit_card_index: String, energy_card_id: String) -> Self {
        AttachGeneralEnergyCardRequestForm {
            session_id: session_id.to_string(),
            unit_card_index: unit_card_index.to_string(),
            energy_card_id: energy_card_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_unit_card_index(&self) -> &str {
        &self.unit_card_index
    }

    pub fn get_energy_card_id(&self) -> &str {
        &self.energy_card_id
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }

    pub fn to_check_protocol_hacking_request(&self, account_unique_id: i32, energy_card_id: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id, energy_card_id)
    }

    pub fn to_is_it_energy_card_request(&self, energy_card_id: i32) -> IsItEnergyCardRequest {
        IsItEnergyCardRequest::new(energy_card_id)
    }

    pub fn to_can_use_card_request(&self, account_unique_id: i32, energy_card_number: i32) -> CanUseCardRequest {
        CanUseCardRequest::new(account_unique_id, energy_card_number)
    }

    pub fn to_use_game_hand_energy_card_request(&self, account_unique_id: i32, energy_card_id: i32) -> UseGameHandEnergyCardRequest {
        UseGameHandEnergyCardRequest::new(
            account_unique_id, energy_card_id)
    }

    pub fn to_place_to_tomb_request(&self, account_unique_id: i32, used_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(account_unique_id, used_card_id)
    }

    pub fn to_summary_energy_card_effect_request(&self, energy_card_id: i32) -> SummaryEnergyCardEffectRequest {
        SummaryEnergyCardEffectRequest::new(
            energy_card_id)
    }

    pub fn to_attach_single_energy_to_field_unit_request(&self, account_unique_id: i32, unit_card_index: i32, race_enum: RaceEnum) -> AttachSingleEnergyToUnitIndexRequest {
        AttachSingleEnergyToUnitIndexRequest::new(account_unique_id, unit_card_index, race_enum)
    }

    pub fn to_get_current_attached_energy_of_unit_by_index_request(
        &self,
        account_unique_id: i32,
        unit_index: i32,) -> GetCurrentAttachedEnergyOfFieldUnitByIndexRequest {

        GetCurrentAttachedEnergyOfFieldUnitByIndexRequest::new(
            account_unique_id, unit_index)
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_notify_opponent_you_use_general_energy_card(&self, opponent_unique_id: i32, usage_energy_card_id: i32, unit_card_index: i32, energy_race: i32, energy_count: i32) -> NotifyOpponentYouUseEnergyCardRequest {
        NotifyOpponentYouUseEnergyCardRequest::new(
            opponent_unique_id, usage_energy_card_id, unit_card_index, energy_race, energy_count)
    }
}
