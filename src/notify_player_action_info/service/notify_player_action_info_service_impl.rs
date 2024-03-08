use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::notify_player_action_info::repository::notify_player_action_info_repository_impl::NotifyPlayerActionInfoRepositoryImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::request::notice_basic_attack_to_main_character_request::NoticeBasicAttackToMainCharacterRequest;
use crate::notify_player_action_info::service::request::notice_basic_attack_to_unit_request::NoticeBasicAttackToUnitRequest;
use crate::notify_player_action_info::service::request::notice_deploy_non_targeting_attack_passive_skill_request::NoticeDeployNonTargetingAttackPassiveSkillRequest;
use crate::notify_player_action_info::service::request::notice_deploy_targeting_attack_passive_skill_to_unit_request::NoticeDeployTargetingAttackPassiveSkillToUnitRequest;
use crate::notify_player_action_info::service::request::notice_deploy_targeting_attack_to_game_main_character_request::NoticeDeployTargetingAttackToGameMainCharacterRequest;
use crate::notify_player_action_info::service::request::notice_my_turn_end_request::NoticeMyTurnEndRequest;
use crate::notify_player_action_info::service::request::notice_non_targeting_attack_active_skill_request::NoticeNonTargetingAttackActiveSkillRequest;
use crate::notify_player_action_info::service::request::notice_targeting_attack_active_skill_to_unit_request::NoticeTargetingAttackActiveSkillToUnitRequest;
use crate::notify_player_action_info::service::request::notice_turn_start_non_targeting_attack_passive_skill_request::NoticeTurnStartNonTargetingAttackPassiveSkillRequest;
use crate::notify_player_action_info::service::request::notice_turn_start_targeting_attack_passive_skill_to_unit_request::NoticeTurnStartTargetingAttackPassiveSkillToUnitRequest;
use crate::notify_player_action_info::service::request::notice_turn_start_targeting_attack_to_game_main_character_request::NoticeTurnStartTargetingAttackToGameMainCharacterRequest;

use crate::notify_player_action_info::service::request::notice_use_catastrophic_damage_item_card_request::NoticeUseCatastrophicDamageItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_draw_support_card_request::NoticeUseDrawSupportCardRequest;
use crate::notify_player_action_info::service::request::notice_use_energy_boost_support_card_to_my_specific_unit_request::NoticeUseEnergyBoostSupportCardToSpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_increase_item_card_request::NoticeUseFieldEnergyIncreaseItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_remove_support_card_request::NoticeUseFieldEnergyRemoveSupportCardRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_to_my_specific_unit_request::NoticeUseFieldEnergyToMySpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_general_energy_card_to_my_specific_unit_request::NoticeUseGeneralEnergyCardToMySpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_instant_unit_death_item_card_request::NoticeUseInstantUnitDeathItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_multiple_unit_damage_item_card_request::NoticeUseMultipleUnitDamageItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_search_deck_support_card_request::{NoticeUseSearchDeckSupportCardRequest};
use crate::notify_player_action_info::service::request::notice_use_special_energy_card_to_unit_request::NoticeUseSpecialEnergyCardToUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_unit_card_request::NoticeUseUnitCardRequest;
use crate::notify_player_action_info::service::request::notice_use_unit_energy_remove_item_card_request::NoticeUseUnitEnergyRemoveItemCardRequest;
use crate::notify_player_action_info::service::response::notice_basic_attack_to_main_character_response::NoticeBasicAttackToMainCharacterResponse;
use crate::notify_player_action_info::service::response::notice_basic_attack_to_unit_response::NoticeBasicAttackToUnitResponse;
use crate::notify_player_action_info::service::response::notice_deploy_non_targeting_attack_passive_skill_response::NoticeDeployNonTargetingAttackPassiveSkillResponse;
use crate::notify_player_action_info::service::response::notice_deploy_targeting_attack_passive_skill_to_unit_response::NoticeDeployTargetingAttackPassiveSkillToUnitResponse;
use crate::notify_player_action_info::service::response::notice_deploy_targeting_attack_to_game_main_character_response::NoticeDeployTargetingAttackToGameMainCharacterResponse;
use crate::notify_player_action_info::service::response::notice_my_turn_end_response::NoticeMyTurnEndResponse;
use crate::notify_player_action_info::service::response::notice_non_targeting_attack_active_skill_response::NoticeNonTargetingAttackActiveSkillResponse;
use crate::notify_player_action_info::service::response::notice_targeting_attack_active_skill_to_unit_response::NoticeTargetingAttackActiveSkillToUnitResponse;
use crate::notify_player_action_info::service::response::notice_turn_start_non_targeting_attack_passive_skill_response::NoticeTurnStartNonTargetingAttackPassiveSkillResponse;
use crate::notify_player_action_info::service::response::notice_turn_start_targeting_attack_passive_skill_to_unit_response::NoticeTurnStartTargetingAttackPassiveSkillToUnitResponse;
use crate::notify_player_action_info::service::response::notice_turn_start_targeting_attack_to_game_main_character_response::NoticeTurnStartTargetingAttackToGameMainCharacterResponse;

use crate::notify_player_action_info::service::response::notice_use_catastrophic_damage_item_card_response::NoticeUseCatastrophicDamageItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_draw_support_card_response::NoticeUseDrawSupportCardResponse;
use crate::notify_player_action_info::service::response::notice_use_energy_boost_support_card_to_my_specific_unit_response::{NoticeUseEnergyBoostSupportCardToSpecificUnitResponse};
use crate::notify_player_action_info::service::response::notice_use_field_energy_increase_item_card_response::NoticeUseFieldEnergyIncreaseItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_field_energy_remove_support_card_response::NoticeUseFieldEnergyRemoveSupportCardResponse;
use crate::notify_player_action_info::service::response::notice_use_field_energy_to_my_specific_unit_response::NoticeUseFieldEnergyToMySpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_general_energy_card_to_my_specific_unit_response::NoticeUseGeneralEnergyCardToMySpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_instant_unit_death_item_card_response::NoticeUseInstantUnitDeathItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_multiple_unit_damage_item_card_response::NoticeUseMultipleUnitDamageItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_search_deck_support_card_response::{NoticeUseSearchDeckSupportCardResponse};
use crate::notify_player_action_info::service::response::notice_use_special_energy_card_to_unit_response::NoticeUseSpecialEnergyCardToUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_unit_card_response::NoticeUseUnitCardResponse;
use crate::notify_player_action_info::service::response::notice_use_unit_energy_remove_item_card_response::NoticeUseUnitEnergyRemoveItemCardResponse;

pub struct NotifyPlayerActionInfoServiceImpl {
    notify_player_action_info_repository: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
}

impl NotifyPlayerActionInfoServiceImpl {
    pub fn new(
        notify_player_action_info_repository: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
    ) -> Self {

        NotifyPlayerActionInfoServiceImpl {
            notify_player_action_info_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionInfoServiceImpl::new(
                            NotifyPlayerActionInfoRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl NotifyPlayerActionInfoService for NotifyPlayerActionInfoServiceImpl {

    async fn notice_use_unit_card(
        &mut self, notice_use_unit_card_request: NoticeUseUnitCardRequest)
        -> NoticeUseUnitCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_unit_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_deploy_unit(
                notice_use_unit_card_request.get_opponent_unique_id(),
                notice_use_unit_card_request.get_player_hand_use_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseUnitCardResponse::new(response)
    }

    async fn notice_use_field_energy_to_my_specific_unit(
        &mut self,
        notice_use_field_energy_to_specific_unit_request: NoticeUseFieldEnergyToMySpecificUnitRequest)
        -> NoticeUseFieldEnergyToMySpecificUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_field_energy_to_my_specific_unit()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_field_energy_to_unit(
                notice_use_field_energy_to_specific_unit_request.get_opponent_unique_id(),
                notice_use_field_energy_to_specific_unit_request.get_player_field_energy_map_for_notice().clone(),
                notice_use_field_energy_to_specific_unit_request.get_player_field_unit_energy_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseFieldEnergyToMySpecificUnitResponse::new(response)
    }

    async fn notice_use_general_energy_card_to_my_specific_unit(
        &mut self, notice_use_general_energy_card_to_my_specific_unit_request: NoticeUseGeneralEnergyCardToMySpecificUnitRequest)
        -> NoticeUseGeneralEnergyCardToMySpecificUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_general_energy_card_to_my_specific_unit()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_general_energy_to_unit(
                    notice_use_general_energy_card_to_my_specific_unit_request.get_opponent_unique_id(),
                    notice_use_general_energy_card_to_my_specific_unit_request.get_player_hand_use_map_for_notice().clone(),
                    notice_use_general_energy_card_to_my_specific_unit_request.get_player_field_unit_energy_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseGeneralEnergyCardToMySpecificUnitResponse::new(response)
    }

    async fn notice_use_special_energy_card_to_unit(
        &mut self, notice_use_special_energy_card_to_unit_request: NoticeUseSpecialEnergyCardToUnitRequest)
        -> NoticeUseSpecialEnergyCardToUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_special_energy_card_to_unit()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_special_energy_to_unit(
                notice_use_special_energy_card_to_unit_request.get_opponent_unique_id(),
                notice_use_special_energy_card_to_unit_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_special_energy_card_to_unit_request.get_player_field_unit_energy_map_for_notice().clone(),
                notice_use_special_energy_card_to_unit_request.get_player_field_unit_extra_effect_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseSpecialEnergyCardToUnitResponse::new(response)
    }

    async fn notice_use_energy_boost_support_card_to_my_specific_unit(
        &mut self,
        notice_use_energy_boost_support_card_to_specific_unit_request: NoticeUseEnergyBoostSupportCardToSpecificUnitRequest)
        -> NoticeUseEnergyBoostSupportCardToSpecificUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_energy_boost_support_card_to_my_specific_unit()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_unit_energy_boost_support(
                notice_use_energy_boost_support_card_to_specific_unit_request.get_opponent_unique_id(),
                notice_use_energy_boost_support_card_to_specific_unit_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_energy_boost_support_card_to_specific_unit_request.get_player_deck_card_use_list_map_for_notice().clone(),
                notice_use_energy_boost_support_card_to_specific_unit_request.get_player_field_unit_energy_map().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseEnergyBoostSupportCardToSpecificUnitResponse::new(response)
    }

    async fn notice_use_draw_support_card(
        &mut self,
        notice_use_draw_support_card_request: NoticeUseDrawSupportCardRequest)
        -> NoticeUseDrawSupportCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_draw_support_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_draw_support(
                notice_use_draw_support_card_request.get_opponent_unique_id(),
                notice_use_draw_support_card_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_draw_support_card_request.get_player_draw_count_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseDrawSupportCardResponse::new(response)
    }

    async fn notice_use_search_deck_support_card(
        &mut self,
        notice_use_search_deck_support_card_request: NoticeUseSearchDeckSupportCardRequest)
        -> NoticeUseSearchDeckSupportCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_search_deck_support_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_search_deck_support(
                notice_use_search_deck_support_card_request.get_opponent_unique_id(),
                notice_use_search_deck_support_card_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_search_deck_support_card_request.get_player_search_count_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseSearchDeckSupportCardResponse::new(response)
    }

    async fn notice_use_field_energy_remove_support_card(
        &mut self,
        notice_use_field_energy_remove_support_card_request: NoticeUseFieldEnergyRemoveSupportCardRequest)
        -> NoticeUseFieldEnergyRemoveSupportCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_field_energy_remove_support_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_field_energy_remove_support(
                notice_use_field_energy_remove_support_card_request.get_opponent_unique_id(),
                notice_use_field_energy_remove_support_card_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_field_energy_remove_support_card_request.get_player_field_energy_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseFieldEnergyRemoveSupportCardResponse::new(response)
    }

    async fn notice_use_instant_unit_death_item_card(
        &mut self,
        notice_use_instant_unit_death_item_card_request: NoticeUseInstantUnitDeathItemCardRequest)
        -> NoticeUseInstantUnitDeathItemCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_instant_unit_death_item_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_instant_unit_death_item(
                notice_use_instant_unit_death_item_card_request.get_opponent_unique_id(),
                notice_use_instant_unit_death_item_card_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_instant_unit_death_item_card_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_use_instant_unit_death_item_card_request.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseInstantUnitDeathItemCardResponse::new(response)
    }

    async fn notice_use_field_energy_increase_item_card(
        &mut self,
        notice_use_field_energy_increase_item_card_request: NoticeUseFieldEnergyIncreaseItemCardRequest)
        -> NoticeUseFieldEnergyIncreaseItemCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_field_energy_increase_item_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_field_energy_increase_item(
                notice_use_field_energy_increase_item_card_request.get_opponent_unique_id(),
                notice_use_field_energy_increase_item_card_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_field_energy_increase_item_card_request.get_player_field_energy_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseFieldEnergyIncreaseItemCardResponse::new(response)
    }

    async fn notice_use_catastrophic_damage_item_card(
        &mut self,
        notice_use_catastrophic_damage_item_card_request: NoticeUseCatastrophicDamageItemCardRequest)
        -> NoticeUseCatastrophicDamageItemCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_catastrophic_damage_item_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_catastrophic_damage_item(
                notice_use_catastrophic_damage_item_card_request.get_opponent_unique_id(),
                notice_use_catastrophic_damage_item_card_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_catastrophic_damage_item_card_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_use_catastrophic_damage_item_card_request.get_player_field_unit_death_map_for_notice().clone(),
                notice_use_catastrophic_damage_item_card_request.get_player_main_character_health_point_map_for_notice().clone(),
                notice_use_catastrophic_damage_item_card_request.get_player_main_character_survival_map_for_notice().clone(),
                notice_use_catastrophic_damage_item_card_request.get_player_deck_card_lost_list_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseCatastrophicDamageItemCardResponse::new(response)
    }

    async fn notice_use_unit_energy_remove_item_card(
        &mut self,
        notice_use_unit_energy_remove_item_card_request: NoticeUseUnitEnergyRemoveItemCardRequest)
        -> NoticeUseUnitEnergyRemoveItemCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_unit_energy_remove_item_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_unit_energy_remove_item(
                notice_use_unit_energy_remove_item_card_request.get_opponent_unique_id(),
                notice_use_unit_energy_remove_item_card_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_unit_energy_remove_item_card_request.get_player_field_unit_energy_map_for_notice().clone(),
                notice_use_unit_energy_remove_item_card_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_use_unit_energy_remove_item_card_request.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseUnitEnergyRemoveItemCardResponse::new(response)
    }

    async fn notice_use_multiple_unit_damage_item_card(
        &mut self,
        notice_use_multiple_unit_damage_item_card_request: NoticeUseMultipleUnitDamageItemCardRequest)
        -> NoticeUseMultipleUnitDamageItemCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_multiple_unit_damage_item_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_use_multiple_unit_damage_item(
                notice_use_multiple_unit_damage_item_card_request.get_opponent_unique_id(),
                notice_use_multiple_unit_damage_item_card_request.get_player_hand_use_map_for_notice().clone(),
                notice_use_multiple_unit_damage_item_card_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_use_multiple_unit_damage_item_card_request.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseMultipleUnitDamageItemCardResponse::new(response)
    }

    async fn notice_basic_attack_to_unit(
        &mut self, notice_basic_attack_to_unit_request: NoticeBasicAttackToUnitRequest)
        -> NoticeBasicAttackToUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_basic_attack_to_unit()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_basic_attack_to_unit(
                notice_basic_attack_to_unit_request.get_opponent_unique_id(),
                notice_basic_attack_to_unit_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_basic_attack_to_unit_request.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                notice_basic_attack_to_unit_request.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeBasicAttackToUnitResponse::new(response)
    }

    async fn notice_basic_attack_to_main_character(
        &mut self, notice_basic_attack_to_main_character_request: NoticeBasicAttackToMainCharacterRequest)
        -> NoticeBasicAttackToMainCharacterResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_basic_attack_to_main_character()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_basic_attack_to_main_character(
                notice_basic_attack_to_main_character_request.get_opponent_unique_id(),
                notice_basic_attack_to_main_character_request.get_player_main_character_health_point_map_for_notice().clone(),
                notice_basic_attack_to_main_character_request.get_player_main_character_survival_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeBasicAttackToMainCharacterResponse::new(response)
    }

    async fn notice_my_turn_end(
        &mut self, notice_my_turn_end_request: NoticeMyTurnEndRequest)
        -> NoticeMyTurnEndResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_my_turn_end()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_turn_end(
                notice_my_turn_end_request.get_opponent_unique_id(),
                notice_my_turn_end_request.get_player_drawn_card_list_map_for_notice().clone(),
                notice_my_turn_end_request.get_player_field_energy_map_for_notice().clone(),
                notice_my_turn_end_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_my_turn_end_request.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                notice_my_turn_end_request.get_player_field_unit_death_map_for_notice().clone(),
                notice_my_turn_end_request.get_player_main_character_survival_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeMyTurnEndResponse::new(response)
    }
    async fn notice_targeting_attack_active_skill_to_unit(
        &mut self, notice_targeting_attack_active_skill_to_unit_request: NoticeTargetingAttackActiveSkillToUnitRequest)
        -> NoticeTargetingAttackActiveSkillToUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_targeting_attack_active_skill_to_unit()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_targeting_attack_active_skill_to_unit(
                notice_targeting_attack_active_skill_to_unit_request.get_opponent_unique_id(),
                notice_targeting_attack_active_skill_to_unit_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_targeting_attack_active_skill_to_unit_request.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                notice_targeting_attack_active_skill_to_unit_request.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeTargetingAttackActiveSkillToUnitResponse::new(response)
    }

    async fn notice_non_targeting_attack_active_skill(
        &mut self, notice_non_targeting_attack_active_skill_request: NoticeNonTargetingAttackActiveSkillRequest)
        -> NoticeNonTargetingAttackActiveSkillResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_non_targeting_attack_active_skill()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_non_targeting_attack_active_skill(
                notice_non_targeting_attack_active_skill_request.get_opponent_unique_id(),
                notice_non_targeting_attack_active_skill_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_non_targeting_attack_active_skill_request.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                notice_non_targeting_attack_active_skill_request.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeNonTargetingAttackActiveSkillResponse::new(response)
    }
    async fn notice_deploy_targeting_attack_passive_skill_to_unit(
        &mut self, notice_deploy_targeting_attack_passive_skill_to_unit_request: NoticeDeployTargetingAttackPassiveSkillToUnitRequest)
        -> NoticeDeployTargetingAttackPassiveSkillToUnitResponse{
        println!("NotifyPlayerActionInfoServiceImpl: notice_deploy_targeting_attack_passive_skill_to_unit()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_deploy_targeting_attack_passive_skill_to_unit(
                notice_deploy_targeting_attack_passive_skill_to_unit_request.get_opponent_unique_id(),
                notice_deploy_targeting_attack_passive_skill_to_unit_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_deploy_targeting_attack_passive_skill_to_unit_request.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                notice_deploy_targeting_attack_passive_skill_to_unit_request.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeDeployTargetingAttackPassiveSkillToUnitResponse::new(response)
    }
    async fn notice_deploy_non_targeting_attack_passive_skill(
        &mut self, notice_deploy_non_targeting_attack_passive_skill: NoticeDeployNonTargetingAttackPassiveSkillRequest)
        -> NoticeDeployNonTargetingAttackPassiveSkillResponse{
        println!("NotifyPlayerActionInfoServiceImpl: notice_deploy_non_targeting_attack_passive_skill()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_deploy_non_targeting_attack_passive_skill(
                notice_deploy_non_targeting_attack_passive_skill.get_opponent_unique_id(),
                notice_deploy_non_targeting_attack_passive_skill.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_deploy_non_targeting_attack_passive_skill.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                notice_deploy_non_targeting_attack_passive_skill.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeDeployNonTargetingAttackPassiveSkillResponse::new(response)
    }
    async fn notice_deploy_targeting_attack_to_game_main_character(
        &mut self, notice_deploy_targeting_attack_to_game_main_character_request: NoticeDeployTargetingAttackToGameMainCharacterRequest)
        -> NoticeDeployTargetingAttackToGameMainCharacterResponse{
        println!("NotifyPlayerActionInfoServiceImpl: notice_deploy_targeting_attack_to_game_main_character()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_deploy_targeting_attack_to_game_main_character(
                notice_deploy_targeting_attack_to_game_main_character_request.get_opponent_unique_id(),
                notice_deploy_targeting_attack_to_game_main_character_request.get_player_main_character_health_point_map_for_notice().clone(),
                notice_deploy_targeting_attack_to_game_main_character_request.get_player_main_character_survival_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeDeployTargetingAttackToGameMainCharacterResponse::new(response)
    }
    async fn notice_turn_start_targeting_attack_passive_skill_to_unit(
        &mut self, notice_turn_start_targeting_attack_passive_skill_to_unit_request: NoticeTurnStartTargetingAttackPassiveSkillToUnitRequest)
        -> NoticeTurnStartTargetingAttackPassiveSkillToUnitResponse{
        println!("NotifyPlayerActionInfoServiceImpl: notice_turn_start_targeting_attack_passive_skill_to_unit()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_turn_start_targeting_attack_passive_skill_to_unit(
                notice_turn_start_targeting_attack_passive_skill_to_unit_request.get_opponent_unique_id(),
                notice_turn_start_targeting_attack_passive_skill_to_unit_request.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_turn_start_targeting_attack_passive_skill_to_unit_request.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                notice_turn_start_targeting_attack_passive_skill_to_unit_request.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeTurnStartTargetingAttackPassiveSkillToUnitResponse::new(response)
    }
    async fn notice_turn_start_non_targeting_attack_passive_skill(
        &mut self, notice_turn_start_non_targeting_attack_passive_skill: NoticeTurnStartNonTargetingAttackPassiveSkillRequest)
        -> NoticeTurnStartNonTargetingAttackPassiveSkillResponse{
        println!("NotifyPlayerActionInfoServiceImpl: notice_turn_start_non_targeting_attack_passive_skill()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_turn_start_non_targeting_attack_passive_skill(
                notice_turn_start_non_targeting_attack_passive_skill.get_opponent_unique_id(),
                notice_turn_start_non_targeting_attack_passive_skill.get_player_field_unit_health_point_map_for_notice().clone(),
                notice_turn_start_non_targeting_attack_passive_skill.get_player_field_unit_harmful_effect_map_for_notice().clone(),
                notice_turn_start_non_targeting_attack_passive_skill.get_player_field_unit_death_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeTurnStartNonTargetingAttackPassiveSkillResponse::new(response)
    }
    async fn notice_turn_start_targeting_attack_to_game_main_character(
        &mut self, notice_turn_start_targeting_attack_to_game_main_character_request: NoticeTurnStartTargetingAttackToGameMainCharacterRequest)
        -> NoticeTurnStartTargetingAttackToGameMainCharacterResponse{
        println!("NotifyPlayerActionInfoServiceImpl: notice_turn_start_targeting_attack_to_game_main_character()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notice_turn_start_targeting_attack_to_game_main_character(
                notice_turn_start_targeting_attack_to_game_main_character_request.get_opponent_unique_id(),
                notice_turn_start_targeting_attack_to_game_main_character_request.get_player_main_character_health_point_map_for_notice().clone(),
                notice_turn_start_targeting_attack_to_game_main_character_request.get_player_main_character_survival_map_for_notice().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeTurnStartTargetingAttackToGameMainCharacterResponse::new(response)
    }
}