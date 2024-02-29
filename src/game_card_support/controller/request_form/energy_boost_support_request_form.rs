use std::collections::HashMap;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::service::request::summarize_support_card_effect_request::SummarizeSupportCardEffectRequest;
use crate::game_card_support_usage_counter::service::request::check_support_card_usage_count_request::CheckSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::update_support_card_usage_count_request::UpdateSupportCardUsageCountRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_deck::service::request::found_card_from_deck_request::FoundCardFromDeckRequest;
use crate::game_deck::service::request::game_deck_card_shuffle_request::GameDeckCardShuffleRequest;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::service::request::attach_multiple_energy_to_unit_index_request::AttachMultipleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::get_current_attached_energy_of_field_unit_by_index_request::GetCurrentAttachedEnergyOfFieldUnitByIndexRequest;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action_info::service::request::notice_boost_energy_to_specific_unit_request::{NoticeBoostEnergyToSpecificUnitRequest};
use crate::notify_player_action_info::service::request::notice_use_energy_boost_support_card_to_my_specific_unit_request::NoticeUseEnergyBoostSupportCardToSpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_hand_card_request::NoticeUseHandCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;
use crate::ui_data_generator::service::request::generate_my_specific_unit_energy_data_request::GenerateMySpecificUnitEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_deck_card_list_data_request::GenerateUseMyDeckCardListDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;

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

    pub fn get_unit_unit_index_number(
        &self) -> &str { &self.unit_index_number }

    pub fn get_support_card_id(
        &self) -> &str { &self.support_card_id }

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

    pub fn to_check_support_card_usage_count_request(
        &self,
        account_unique_id: i32) -> CheckSupportCardUsageCountRequest {

        CheckSupportCardUsageCountRequest::new(
            account_unique_id)
    }

    pub fn to_update_support_card_usage_count_request(
        &self,
        account_unique_id: i32) -> UpdateSupportCardUsageCountRequest {

        UpdateSupportCardUsageCountRequest::new(
            account_unique_id)
    }
    pub fn to_use_game_hand_support_card_request(
        &self,
        account_unique_id: i32,
        support_card_number: i32) -> UseGameHandSupportCardRequest {

        UseGameHandSupportCardRequest::new(
            account_unique_id,
            support_card_number)
    }

    pub fn to_place_to_tomb_request(
        &self,
        account_unique_id: i32,
        used_card_id: i32) -> PlaceToTombRequest {

        PlaceToTombRequest::new(
            account_unique_id,
            used_card_id)
    }

    pub fn to_summarize_support_card_effect_request(
        &self,
        support_card_number: i32) -> SummarizeSupportCardEffectRequest {

        SummarizeSupportCardEffectRequest::new(
            support_card_number)
    }

    pub fn to_found_card_from_deck_request(
        &self,
        account_unique_id: i32,
        need_to_find_card_id: i32,
        energy_count: i32) -> FoundCardFromDeckRequest {

        FoundCardFromDeckRequest::new(
            account_unique_id,
            need_to_find_card_id,
            energy_count)
    }

    pub fn to_attach_multiple_energy_to_unit_index_request(
        &self,
        account_unique_id: i32,
        unit_number: i32,
        boost_race: RaceEnum,
        energy_count: i32) -> AttachMultipleEnergyToUnitIndexRequest {

        AttachMultipleEnergyToUnitIndexRequest::new(
            account_unique_id,
            unit_number,
            boost_race,
            energy_count)
    }

    pub fn to_get_current_attached_energy_of_field_unit_by_index_request(
        &self,
        account_unique_id: i32,
        field_unit_index: i32) -> GetCurrentAttachedEnergyOfFieldUnitByIndexRequest {

        GetCurrentAttachedEnergyOfFieldUnitByIndexRequest::new(
            account_unique_id,
            field_unit_index)
    }

    pub fn to_shuffle_deck_request(
        &self) -> GameDeckCardShuffleRequest {

        GameDeckCardShuffleRequest::new(
            self.session_id.clone())
    }

    pub fn to_find_opponent_by_account_id_request(
        &self,
        account_unique_id: i32) -> FindOpponentByAccountIdRequest {

        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_generate_use_my_hand_card_data_request(
        &self,
        used_hand_card_id: i32
    ) -> GenerateUseMyHandCardDataRequest {

        GenerateUseMyHandCardDataRequest::new(
            used_hand_card_id)
    }

    pub fn to_generate_use_my_deck_card_list_data_request(
        &self,
        deck_card_id_list: Vec<i32>
    ) -> GenerateUseMyDeckCardListDataRequest {

        GenerateUseMyDeckCardListDataRequest::new(
            deck_card_id_list)
    }

    pub fn to_generate_my_specific_unit_energy_data_request(
        &self,
        unit_index: i32,
        updated_unit_energy_map: AttachedEnergyMap) -> GenerateMySpecificUnitEnergyDataRequest {

        GenerateMySpecificUnitEnergyDataRequest::new(
            unit_index,
            updated_unit_energy_map)
    }

    pub fn to_notice_energy_boost_support_card_to_specific_unit_request(
        &self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
        player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> NoticeUseEnergyBoostSupportCardToSpecificUnitRequest {

        NoticeUseEnergyBoostSupportCardToSpecificUnitRequest::new(
            opponent_unique_id,
            player_hand_use_map_for_notice,
            player_deck_card_use_list_map_for_notice,
            player_field_unit_energy_map)
    }
}