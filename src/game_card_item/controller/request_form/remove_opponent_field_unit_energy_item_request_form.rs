use std::collections::HashMap;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::service::request::apply_damage_to_target_unit_index_request::ApplyDamageToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::detach_multiple_energy_from_field_unit_request::DetachMultipleEnergyFromFieldUnitRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::get_current_attached_energy_of_field_unit_by_index_request::GetCurrentAttachedEnergyOfFieldUnitByIndexRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_field_unit_by_index_request::GetCurrentHealthPointOfFieldUnitByIndexRequest;
use crate::game_field_unit::service::request::judge_death_of_unit_request::JudgeDeathOfUnitRequest;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_item_card_request::IsItItemCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action_info::service::request::notice_use_unit_energy_remove_item_card_request::NoticeUseUnitEnergyRemoveItemCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_death_data_request::GenerateOpponentSpecificUnitDeathDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_energy_data_request::GenerateOpponentSpecificUnitEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_health_point_data_request::GenerateOpponentSpecificUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_health_point_data_response::GenerateOpponentSpecificUnitHealthPointDataResponse;

#[derive(Debug)]
pub struct RemoveOpponentFieldUnitEnergyItemRequestForm {
    session_id: String,
    item_card_id: String,
    opponent_target_unit_index: String,
}

impl RemoveOpponentFieldUnitEnergyItemRequestForm {
    pub fn new(session_id: &str, item_card_id: &str, opponent_target_unit_index: &str) -> Self {
        RemoveOpponentFieldUnitEnergyItemRequestForm {
            session_id: session_id.to_string(),
            item_card_id: item_card_id.to_string(),
            opponent_target_unit_index: opponent_target_unit_index.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_item_card_id(&self) -> &str {
        &self.item_card_id
    }

    pub fn get_opponent_target_unit_index(&self) -> &str { &self.opponent_target_unit_index }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }

    pub fn to_check_protocol_hacking_request(&self,
                                             account_unique_id: i32,
                                             support_card_number: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id,
                                         support_card_number)
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

    pub fn to_find_opponent_by_account_id_request(&self,
                                                  account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(account_unique_id)
    }

    pub fn to_find_target_unit_id_by_index_request(&self,
                                                   account_unique_id: i32,
                                                   field_unit_index: i32) -> FindTargetUnitIdByIndexRequest {
        FindTargetUnitIdByIndexRequest::new(account_unique_id,
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

    pub fn to_get_current_health_point_of_field_unit_by_index_request(
        &self,
        account_unique_id: i32,
        unit_index: i32) -> GetCurrentHealthPointOfFieldUnitByIndexRequest {

        GetCurrentHealthPointOfFieldUnitByIndexRequest::new(
            account_unique_id,
            unit_index)
    }

    pub fn to_judge_death_of_unit_request(
        &self,
        account_unique_id: i32,
        unit_index: i32) -> JudgeDeathOfUnitRequest {

        JudgeDeathOfUnitRequest::new(
            account_unique_id,
            unit_index)
    }

    pub fn to_get_current_attached_energy_of_unit_by_index_request(&self,
                                                                   account_unique_id: i32,
                                                                   unit_index: i32,) -> GetCurrentAttachedEnergyOfFieldUnitByIndexRequest {
        GetCurrentAttachedEnergyOfFieldUnitByIndexRequest::new(
            account_unique_id, unit_index)
    }

    pub fn to_detach_energy_from_field_unit_request(&self,
                                                    opponent_unique_id: i32,
                                                    opponent_target_unit_index: i32,
                                                    race_enum: RaceEnum,
                                                    quantity: i32) -> DetachMultipleEnergyFromFieldUnitRequest {
        DetachMultipleEnergyFromFieldUnitRequest::new(
            opponent_unique_id,
            opponent_target_unit_index,
            race_enum,
            quantity)
    }

    pub fn to_generate_use_my_hand_card_data_request(
        &self,
        used_hand_card_id: i32
    ) -> GenerateUseMyHandCardDataRequest {

        GenerateUseMyHandCardDataRequest::new(
            used_hand_card_id)
    }

    pub fn to_generate_opponent_specific_unit_energy_data_request(
        &self,
        opponent_unit_index: i32,
        updated_opponent_unit_energy_map: AttachedEnergyMap) -> GenerateOpponentSpecificUnitEnergyDataRequest {

        GenerateOpponentSpecificUnitEnergyDataRequest::new(
            opponent_unit_index,
            updated_opponent_unit_energy_map)
    }

    pub fn to_generate_opponent_specific_unit_health_point_data_request(
        &self,
        opponent_unit_index: i32,
        updated_opponent_unit_health_point: i32
    ) -> GenerateOpponentSpecificUnitHealthPointDataRequest {

        GenerateOpponentSpecificUnitHealthPointDataRequest::new(
            opponent_unit_index,
            updated_opponent_unit_health_point)
    }

    pub fn to_generate_opponent_specific_unit_death_data_request(
        &self,
        opponent_dead_unit_index: i32
    ) -> GenerateOpponentSpecificUnitDeathDataRequest {

        GenerateOpponentSpecificUnitDeathDataRequest::new(
            opponent_dead_unit_index)
    }

    pub fn to_notice_use_unit_energy_remove_item_card_request(
        &self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> NoticeUseUnitEnergyRemoveItemCardRequest {

        NoticeUseUnitEnergyRemoveItemCardRequest::new(
            opponent_unique_id,
            player_hand_use_map_for_notice,
            player_field_unit_energy_map_for_notice,
            player_field_unit_health_point_map_for_notice,
            player_field_unit_death_map_for_notice
        )
    }
}