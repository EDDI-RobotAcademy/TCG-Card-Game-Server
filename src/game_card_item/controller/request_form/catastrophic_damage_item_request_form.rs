use std::collections::HashMap;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_field_unit::service::request::apply_catastrophic_damage_to_field_unit_request::ApplyCatastrophicDamageToFieldUnitRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_all_field_unit_request::GetCurrentHealthPointOfAllFieldUnitRequest;
use crate::game_field_unit::service::request::judge_death_of_every_unit_request::JudgeDeathOfEveryUnitRequest;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_lost_zone::service::request::place_card_to_lost_zone_request::PlaceCardToLostZoneRequest;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::game_main_character::service::request::apply_damage_to_main_character_request::ApplyDamageToMainCharacterRequest;
use crate::game_main_character::service::request::check_main_character_of_account_unique_id_request::CheckMainCharacterOfAccountUniqueIdRequest;
use crate::game_main_character::service::request::get_current_main_character_health_point_request::GetCurrentMainCharacterHealthPointRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_item_card_request::IsItItemCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action_info::service::request::notice_use_catastrophic_damage_item_card_request::NoticeUseCatastrophicDamageItemCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;
use crate::ui_data_generator::service::request::generate_opponent_deck_card_lost_data_request::GenerateOpponentDeckCardLostDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_main_character_health_point_data_request::GenerateOpponentMainCharacterHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_main_character_survival_data_request::GenerateOpponentMainCharacterSurvivalDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_multiple_unit_death_data_request::GenerateOpponentMultipleUnitDeathDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_multiple_unit_health_point_data_request::GenerateOpponentMultipleUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;

#[derive(Debug)]
pub struct CatastrophicDamageItemRequestForm {
    session_id: String,
    item_card_id: String,
}

impl CatastrophicDamageItemRequestForm {
    pub fn new(session_id: &str, item_card_id: &str) -> Self {
        CatastrophicDamageItemRequestForm {
            session_id: session_id.to_string(),
            item_card_id: item_card_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_item_card_id(&self) -> &str {
        &self.item_card_id
    }

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

    pub fn to_is_it_item_card_request(&self, item_card_id: i32) -> IsItItemCardRequest {
        IsItItemCardRequest::new(item_card_id)
    }

    pub fn to_can_use_card_request(&self, account_unique_id: i32, item_card_id: i32) -> CanUseCardRequest {
        CanUseCardRequest::new(account_unique_id, item_card_id)
    }

    pub fn to_summary_item_effect_request(&self, item_card_id: i32) -> SummaryItemCardEffectRequest {
        SummaryItemCardEffectRequest::new(item_card_id)
    }

    pub fn to_use_game_hand_item_card_request(&self, account_unique_id: i32, item_card_id: i32) -> UseGameHandItemCardRequest {
        UseGameHandItemCardRequest::new(
            account_unique_id, item_card_id)
    }

    pub fn to_place_to_tomb_request(&self, account_unique_id: i32, used_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(account_unique_id, used_card_id)
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_find_target_unit_id_by_index_request(&self, opponent_unique_id: i32, opponent_target_unit_index: i32) -> FindTargetUnitIdByIndexRequest {
        FindTargetUnitIdByIndexRequest::new(
            opponent_unique_id, opponent_target_unit_index)
    }

    pub fn to_draw_cards_from_deck_request(&self, opponent_unique_id: i32, draw_count: i32) -> DrawCardsFromDeckRequest {
        DrawCardsFromDeckRequest::new(opponent_unique_id, draw_count)
    }

    pub fn to_apply_catastrophic_damage_to_field_unit_request(&self, opponent_unique_id: i32, damage: i32) -> ApplyCatastrophicDamageToFieldUnitRequest {
        ApplyCatastrophicDamageToFieldUnitRequest::new(opponent_unique_id, damage)
    }

    pub fn to_get_current_health_point_of_all_unit_request(
        &self,
        account_unique_id: i32) -> GetCurrentHealthPointOfAllFieldUnitRequest {

        GetCurrentHealthPointOfAllFieldUnitRequest::new(account_unique_id)
    }

    pub fn to_judge_death_of_every_field_unit_request(
        &self,
        account_unique_id: i32) -> JudgeDeathOfEveryUnitRequest {

        JudgeDeathOfEveryUnitRequest::new(account_unique_id)
    }

    pub fn to_apply_damage_to_main_character(
        &self,
        opponent_unique_id: i32,
        damage: i32) -> ApplyDamageToMainCharacterRequest {

        ApplyDamageToMainCharacterRequest::new(
            opponent_unique_id,
            damage)
    }

    pub fn to_get_current_health_point_of_main_character_request(
        &self,
        account_unique_id: i32) -> GetCurrentMainCharacterHealthPointRequest {

        GetCurrentMainCharacterHealthPointRequest::new(
            account_unique_id)
    }

    pub fn to_check_main_character_survival_request(
        &self,
        account_unique_id: i32) -> CheckMainCharacterOfAccountUniqueIdRequest {

        CheckMainCharacterOfAccountUniqueIdRequest::new(account_unique_id)
    }

    pub fn to_place_card_to_lost_zone_request(&self, opponent_unique_id: i32, will_be_lost_card: i32) -> PlaceCardToLostZoneRequest {
        PlaceCardToLostZoneRequest::new(opponent_unique_id, will_be_lost_card)
    }

    pub fn to_generate_use_my_hand_card_data_request(
        &self,
        used_hand_card_id: i32
    ) -> GenerateUseMyHandCardDataRequest {

        GenerateUseMyHandCardDataRequest::new(
            used_hand_card_id)
    }

    pub fn to_generate_opponent_multiple_unit_health_point_data_request(
        &self,
        opponent_unit_health_point_tuple_list: Vec<(i32, i32)>
    ) -> GenerateOpponentMultipleUnitHealthPointDataRequest {

        GenerateOpponentMultipleUnitHealthPointDataRequest::new(
            opponent_unit_health_point_tuple_list)
    }

    pub fn to_generate_opponent_multiple_health_point_data_request(
        &self,
        health_point_tuple_list: Vec<(i32, i32)>
    ) -> GenerateOpponentMultipleUnitHealthPointDataRequest {

        GenerateOpponentMultipleUnitHealthPointDataRequest::new(
            health_point_tuple_list)
    }

    pub fn to_generate_opponent_multiple_unit_death_data_request(
        &self,
        dead_unit_index_list: Vec<i32>
    ) -> GenerateOpponentMultipleUnitDeathDataRequest {

        GenerateOpponentMultipleUnitDeathDataRequest::new(
            dead_unit_index_list)
    }

    pub fn to_generate_opponent_main_character_health_point_data_request(
        &self,
        main_character_health_point: i32
    ) -> GenerateOpponentMainCharacterHealthPointDataRequest {

        GenerateOpponentMainCharacterHealthPointDataRequest::new(
            main_character_health_point)
    }

    pub fn to_generate_opponent_main_character_survival_data_request(
        &self,
        main_character_status: StatusMainCharacterEnum
    ) -> GenerateOpponentMainCharacterSurvivalDataRequest {

        GenerateOpponentMainCharacterSurvivalDataRequest::new(
            main_character_status)
    }

    pub fn to_generate_opponent_deck_card_lost_data_request(
        &self,
        lost_deck_card_list: Vec<i32>
    ) -> GenerateOpponentDeckCardLostDataRequest {

        GenerateOpponentDeckCardLostDataRequest::new(
            lost_deck_card_list)
    }

    pub fn to_notice_use_catastrophic_item_card_request(
        &self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
        player_deck_card_lost_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
    ) -> NoticeUseCatastrophicDamageItemCardRequest {

        NoticeUseCatastrophicDamageItemCardRequest::new(
            opponent_unique_id,
            player_hand_use_map_for_notice,
            player_field_unit_health_point_map_for_notice,
            player_field_unit_death_map_for_notice,
            player_main_character_health_point_map_for_notice,
            player_main_character_survival_map_for_notice,
            player_deck_card_lost_list_map_for_notice
        )
    }
}