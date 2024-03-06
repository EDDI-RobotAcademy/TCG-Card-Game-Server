use async_trait::async_trait;
use crate::notify_player_action_info::service::request::notice_basic_attack_to_main_character_request::NoticeBasicAttackToMainCharacterRequest;
use crate::notify_player_action_info::service::request::notice_basic_attack_to_unit_request::NoticeBasicAttackToUnitRequest;
use crate::notify_player_action_info::service::request::notice_my_turn_end_request::NoticeMyTurnEndRequest;
use crate::notify_player_action_info::service::request::notice_non_targeting_attack_active_skill_request::NoticeNonTargetingAttackActiveSkillRequest;
use crate::notify_player_action_info::service::request::notice_targeting_attack_active_skill_to_unit_request::NoticeTargetingAttackActiveSkillToUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_catastrophic_damage_item_card_request::NoticeUseCatastrophicDamageItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_draw_support_card_request::NoticeUseDrawSupportCardRequest;
use crate::notify_player_action_info::service::request::notice_use_energy_boost_support_card_to_my_specific_unit_request::NoticeUseEnergyBoostSupportCardToSpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_increase_item_card_request::NoticeUseFieldEnergyIncreaseItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_remove_support_card_request::NoticeUseFieldEnergyRemoveSupportCardRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_to_my_specific_unit_request::NoticeUseFieldEnergyToMySpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_general_energy_card_to_my_specific_unit_request::NoticeUseGeneralEnergyCardToMySpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_instant_unit_death_item_card_request::NoticeUseInstantUnitDeathItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_multiple_unit_damage_item_card_request::NoticeUseMultipleUnitDamageItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_search_deck_support_card_request::NoticeUseSearchDeckSupportCardRequest;
use crate::notify_player_action_info::service::request::notice_use_special_energy_card_to_unit_request::NoticeUseSpecialEnergyCardToUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_unit_card_request::NoticeUseUnitCardRequest;
use crate::notify_player_action_info::service::request::notice_use_unit_energy_remove_item_card_request::NoticeUseUnitEnergyRemoveItemCardRequest;
use crate::notify_player_action_info::service::response::notice_basic_attack_to_main_character_response::NoticeBasicAttackToMainCharacterResponse;
use crate::notify_player_action_info::service::response::notice_basic_attack_to_unit_response::NoticeBasicAttackToUnitResponse;
use crate::notify_player_action_info::service::response::notice_my_turn_end_response::NoticeMyTurnEndResponse;
use crate::notify_player_action_info::service::response::notice_non_targeting_attack_active_skill_response::NoticeNonTargetingAttackActiveSkillResponse;
use crate::notify_player_action_info::service::response::notice_targeting_attack_active_skill_to_unit_response::NoticeTargetingAttackActiveSkillToUnitResponse;

use crate::notify_player_action_info::service::response::notice_use_catastrophic_damage_item_card_response::NoticeUseCatastrophicDamageItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_draw_support_card_response::NoticeUseDrawSupportCardResponse;
use crate::notify_player_action_info::service::response::notice_use_energy_boost_support_card_to_my_specific_unit_response::NoticeUseEnergyBoostSupportCardToSpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_field_energy_increase_item_card_response::NoticeUseFieldEnergyIncreaseItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_field_energy_remove_support_card_response::NoticeUseFieldEnergyRemoveSupportCardResponse;
use crate::notify_player_action_info::service::response::notice_use_field_energy_to_my_specific_unit_response::NoticeUseFieldEnergyToMySpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_general_energy_card_to_my_specific_unit_response::NoticeUseGeneralEnergyCardToMySpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_instant_unit_death_item_card_response::NoticeUseInstantUnitDeathItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_multiple_unit_damage_item_card_response::NoticeUseMultipleUnitDamageItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_search_deck_support_card_response::NoticeUseSearchDeckSupportCardResponse;
use crate::notify_player_action_info::service::response::notice_use_special_energy_card_to_unit_response::NoticeUseSpecialEnergyCardToUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_unit_card_response::NoticeUseUnitCardResponse;
use crate::notify_player_action_info::service::response::notice_use_unit_energy_remove_item_card_response::NoticeUseUnitEnergyRemoveItemCardResponse;

#[async_trait]
pub trait NotifyPlayerActionInfoService {
    async fn notice_use_unit_card(
        &mut self, notice_use_unit_card_request: NoticeUseUnitCardRequest)
        -> NoticeUseUnitCardResponse;
    async fn notice_use_field_energy_to_my_specific_unit(
        &mut self,
        notice_use_field_energy_to_specific_unit_request: NoticeUseFieldEnergyToMySpecificUnitRequest)
        -> NoticeUseFieldEnergyToMySpecificUnitResponse;
    async fn notice_use_general_energy_card_to_my_specific_unit(
        &mut self, notice_use_general_energy_card_to_my_specific_unit_request: NoticeUseGeneralEnergyCardToMySpecificUnitRequest)
        -> NoticeUseGeneralEnergyCardToMySpecificUnitResponse;
    async fn notice_use_special_energy_card_to_unit(
        &mut self, notice_use_special_energy_card_to_unit_request: NoticeUseSpecialEnergyCardToUnitRequest)
        -> NoticeUseSpecialEnergyCardToUnitResponse;
    async fn notice_use_energy_boost_support_card_to_my_specific_unit(
        &mut self,
        notice_use_energy_boost_support_card_to_specific_unit_request: NoticeUseEnergyBoostSupportCardToSpecificUnitRequest)
        -> NoticeUseEnergyBoostSupportCardToSpecificUnitResponse;
    async fn notice_use_draw_support_card(
        &mut self,
        notice_use_draw_support_card_request: NoticeUseDrawSupportCardRequest)
        -> NoticeUseDrawSupportCardResponse;
    async fn notice_use_search_deck_support_card(
        &mut self,
        notice_use_search_deck_support_card_request: NoticeUseSearchDeckSupportCardRequest)
        -> NoticeUseSearchDeckSupportCardResponse;
    async fn notice_use_field_energy_remove_support_card(
        &mut self,
        notice_use_field_energy_remove_support_card_request: NoticeUseFieldEnergyRemoveSupportCardRequest)
        -> NoticeUseFieldEnergyRemoveSupportCardResponse;
    async fn notice_use_instant_unit_death_item_card(
        &mut self,
        notice_use_instant_unit_death_item_card_request: NoticeUseInstantUnitDeathItemCardRequest)
        -> NoticeUseInstantUnitDeathItemCardResponse;
    async fn notice_use_field_energy_increase_item_card(
        &mut self,
        notice_use_field_energy_increase_item_card_request: NoticeUseFieldEnergyIncreaseItemCardRequest)
        -> NoticeUseFieldEnergyIncreaseItemCardResponse;
    async fn notice_use_catastrophic_damage_item_card(
        &mut self,
        notice_use_catastrophic_damage_item_card_request: NoticeUseCatastrophicDamageItemCardRequest)
        -> NoticeUseCatastrophicDamageItemCardResponse;
    async fn notice_use_unit_energy_remove_item_card(
        &mut self,
        notice_use_unit_energy_remove_item_card_request: NoticeUseUnitEnergyRemoveItemCardRequest)
        -> NoticeUseUnitEnergyRemoveItemCardResponse;
    async fn notice_use_multiple_unit_damage_item_card(
        &mut self,
        notice_use_multiple_unit_damage_item_card_request: NoticeUseMultipleUnitDamageItemCardRequest)
        -> NoticeUseMultipleUnitDamageItemCardResponse;
    async fn notice_basic_attack_to_unit(
        &mut self, notice_basic_attack_to_unit_request: NoticeBasicAttackToUnitRequest)
        -> NoticeBasicAttackToUnitResponse;
    async fn notice_basic_attack_to_main_character(
        &mut self, notice_basic_attack_to_main_character_request: NoticeBasicAttackToMainCharacterRequest)
        -> NoticeBasicAttackToMainCharacterResponse;
    async fn notice_my_turn_end(
        &mut self, notice_my_turn_end_request: NoticeMyTurnEndRequest)
        -> NoticeMyTurnEndResponse;
    async fn notice_targeting_attack_active_skill_to_unit(
        &mut self, notice_targeting_attack_active_skill_to_unit_request: NoticeTargetingAttackActiveSkillToUnitRequest)
        -> NoticeTargetingAttackActiveSkillToUnitResponse;
    async fn notice_non_targeting_attack_active_skill(
        &mut self, notice_non_targeting_attack_active_skill_request: NoticeNonTargetingAttackActiveSkillRequest)
        -> NoticeNonTargetingAttackActiveSkillResponse;
}