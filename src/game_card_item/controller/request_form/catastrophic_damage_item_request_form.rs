use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_deck::service::request::game_deck_card_draw_request::GameDeckCardDrawRequest;
use crate::game_field_unit::service::request::apply_catastrophic_damage_to_field_unit_request::ApplyCatastrophicDamageToFieldUnitRequest;
use crate::game_field_unit::service::request::apply_damage_to_target_unit_index_request::ApplyDamageToTargetUnitIndexRequest;
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
use crate::game_winner_check::service::request::check_main_character_request::CheckMainCharacterRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_catastrophic_damage_item_card_request::NotifyToOpponentYouUseCatastrophicDamageItemCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_damage_main_character_item_card_request::NotifyToOpponentYouUseDamageMainCharacterItemCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_destroy_deck_item_card_request::NotifyToOpponentYouUseDestroyDeckItemCardRequest;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_every_opponent_unit_request::NoticeApplyDamageToEveryOpponentUnitRequest;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_opponent_main_character_request::NoticeApplyDamageToOpponentMainCharacterRequest;
use crate::notify_player_action_info::service::request::notice_lost_deck_card_of_opponent_request::NoticeLostDeckCardOfOpponentRequest;
use crate::notify_player_action_info::service::request::notice_use_hand_card_request::NoticeUseHandCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

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

    pub fn to_notice_use_hand_card_request(
        &self,
        opponent_unique_id: i32,
        used_hand_card_id: i32
    ) -> NoticeUseHandCardRequest {

        NoticeUseHandCardRequest::new(
            opponent_unique_id,
            used_hand_card_id)
    }

    pub fn to_notice_apply_damage_to_every_opponent_unit_request(
        &self,
        opponent_unique_id: i32,
        damage: i32,
        updated_health_point_list: Vec<i32>,
        dead_unit_index_list: Vec<i32>
    ) -> NoticeApplyDamageToEveryOpponentUnitRequest {

        NoticeApplyDamageToEveryOpponentUnitRequest::new(
            opponent_unique_id,
            damage,
            updated_health_point_list,
            dead_unit_index_list)
    }

    pub fn to_notice_apply_damage_to_opponent_main_character_request(
        &self,
        opponent_unique_id: i32,
        damage: i32,
        updated_health_point: i32,
        opponent_survival_status: StatusMainCharacterEnum,) -> NoticeApplyDamageToOpponentMainCharacterRequest {

        NoticeApplyDamageToOpponentMainCharacterRequest::new(
            opponent_unique_id,
            damage,
            updated_health_point,
            opponent_survival_status)
    }

    pub fn to_notice_lost_deck_card_of_opponent_request(
        &self,
        opponent_unique_id: i32,
        lost_deck_card_list: Vec<i32>) -> NoticeLostDeckCardOfOpponentRequest {

        NoticeLostDeckCardOfOpponentRequest::new(
            opponent_unique_id,
            lost_deck_card_list)
    }

    pub fn to_notify_opponent_you_use_catastrophic_damage_item_card_request(&self, opponent_unique_id: i32, item_card_id: i32, damage_for_field_unit: i32) -> NotifyToOpponentYouUseCatastrophicDamageItemCardRequest {
        NotifyToOpponentYouUseCatastrophicDamageItemCardRequest::new(opponent_unique_id, item_card_id, damage_for_field_unit)
    }
    pub fn to_notify_opponent_you_use_damage_main_character_item_card_request(&self, opponent_unique_id: i32, item_card_id: i32, damage_for_main_character: i32) -> NotifyToOpponentYouUseDamageMainCharacterItemCardRequest {
        NotifyToOpponentYouUseDamageMainCharacterItemCardRequest::new(opponent_unique_id, item_card_id, damage_for_main_character)
    }
    pub fn to_notify_opponent_you_use_destroy_deck_item_card_request(&self, opponent_unique_id: i32, item_card_id: i32, will_be_lost_card: i32) -> NotifyToOpponentYouUseDestroyDeckItemCardRequest {
        NotifyToOpponentYouUseDestroyDeckItemCardRequest::new(opponent_unique_id, item_card_id, will_be_lost_card)
    }
}