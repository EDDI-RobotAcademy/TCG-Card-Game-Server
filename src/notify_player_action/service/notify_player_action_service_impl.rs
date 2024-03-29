use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::notify_player_action::repository::notify_player_action_repository::NotifyPlayerActionRepository;
use crate::notify_player_action::repository::notify_player_action_repository_impl::NotifyPlayerActionRepositoryImpl;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::request::notify_to_opponent_you_attached_field_energy_to_field_unit_request::NotifyOpponentYouAttachedFieldEnergyRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_deploy_unit_request::NotifyToOpponentYouDeployUnitRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_draw_support_card_request::NotifyToOpponentYouUseDrawSupportCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_boost_card_request::NotifyToOpponentYouUseEnergyBoostCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_card_request::{NotifyOpponentYouUseEnergyCardRequest};
use crate::notify_player_action::service::request::notify_to_opponent_you_use_field_energy_remove_support_card_request::NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_item_field_energy_increase_request::NotifyOpponentYouUseItemFieldEnergyIncreaseRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_item_instant_death_request::NotifyToOpponentYouUseItemInstantDeathRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_item_instant_death_alternatives_request::NotifyToOpponentYouUseItemInstantDeathAlternativesRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_tool_card_to_enhance_attack_point_request::NotifyToOpponentYouUseToolCardToEnhanceAttackPointRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_search_support_card_request::NotifyOpponentYouUseSearchSupportRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_catastrophic_damage_item_card_request::NotifyToOpponentYouUseCatastrophicDamageItemCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_damage_main_character_item_card_request::NotifyToOpponentYouUseDamageMainCharacterItemCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_destroy_deck_item_card_request::NotifyToOpponentYouUseDestroyDeckItemCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_field_unit_energy_removal_item_card_request::NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_hand_card_request::NotifyOpponentYouUseHandCardRequest;
use crate::notify_player_action::service::response::notify_to_opponent_you_attached_field_energy_to_field_unit_response::NotifyOpponentYouAttachedFieldEnergyResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_deploy_unit_response::NotifyToOpponentYouDeployUnitResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_draw_support_card_response::NotifyToOpponentYouUseDrawSupportCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_energy_boost_card_response::NotifyToOpponentYouUseEnergyBoostCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_energy_card_response::NotifyToOpponentYouUseEnergyCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_item_card_response::NotifyToOpponentYouUseItemCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_item_instant_death_alternatives_response::NotifyToOpponentYouUseItemInstantDeathAlternativesResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_support_card_response::NotifyOpponentYouUseSupportCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_tool_card_to_enhance_attack_point_response::NotifyToOpponentYouUseToolCardToEnhanceAttackPointResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_catastrophic_damage_item_card_response::NotifyToOpponentYouUseCatastrophicDamageItemCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_damage_main_character_item_card_response::NotifyToOpponentYouUseDamageMainCharacterItemCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_destroy_deck_item_card_response::NotifyToOpponentYouUseDestroyDeckItemCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_field_unit_energy_removal_item_card_response::NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_hand_card_response::NotifyOpponentYouUseHandCardResponse;

pub struct NotifyPlayerActionServiceImpl {
    notify_player_action_repository: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>>,
}

impl NotifyPlayerActionServiceImpl {
    pub fn new(
        notify_player_action_repository: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>>,
    ) -> Self {

        NotifyPlayerActionServiceImpl {
            notify_player_action_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionServiceImpl::new(
                            NotifyPlayerActionRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl NotifyPlayerActionService for NotifyPlayerActionServiceImpl {
    async fn notify_opponent_you_deploy_unit(&mut self, notify_to_opponent_what_you_do_request: NotifyToOpponentYouDeployUnitRequest) -> NotifyToOpponentYouDeployUnitResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_deploy_unit()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_to_opponent_what_you_do(
            notify_to_opponent_what_you_do_request.get_opponent_unique_id(),
            notify_to_opponent_what_you_do_request.get_usage_hand_card_id()).await;

        NotifyToOpponentYouDeployUnitResponse::new(notify_to_opponent_what_you_do_response)
    }

    // TODO: 추후 이렇게 기능단위로 다 쪼개는 것이 좋으나 현 시점에서 제약이 많다고 판단하여 보류
    async fn notify_opponent_you_use_hand_card(&mut self, notify_opponent_you_use_hand_card_request: NotifyOpponentYouUseHandCardRequest) -> NotifyOpponentYouUseHandCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_opponent_you_use_hand_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_to_opponent_what_you_do(
            notify_opponent_you_use_hand_card_request.get_opponent_unique_id(),
            notify_opponent_you_use_hand_card_request.get_usage_hand_card_id()).await;

        NotifyOpponentYouUseHandCardResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_opponent_you_attach_field_energy_to_field_unit(&mut self, notify_opponent_you_attached_field_energy_request: NotifyOpponentYouAttachedFieldEnergyRequest) -> NotifyOpponentYouAttachedFieldEnergyResponse {
        println!("NotifyPlayerActionServiceImpl: notify_opponent_you_attach_field_energy_to_field_unit()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_to_opponent_you_attached_field_energy_to_field_unit(
            notify_opponent_you_attached_field_energy_request.get_opponent_unique_id(),
            notify_opponent_you_attached_field_energy_request.get_unit_card_index(),
            notify_opponent_you_attached_field_energy_request.get_energy_race(),
            notify_opponent_you_attached_field_energy_request.get_energy_count(),
            notify_opponent_you_attached_field_energy_request.get_current_unit_energy_count(),
            notify_opponent_you_attached_field_energy_request.get_remaining_field_energy()).await;

        NotifyOpponentYouAttachedFieldEnergyResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_opponent_you_use_energy_card(&mut self, notify_to_opponent_you_use_energy_card_request: NotifyOpponentYouUseEnergyCardRequest) -> NotifyToOpponentYouUseEnergyCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_opponent_you_use_energy_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_to_opponent_you_use_energy_card(
            notify_to_opponent_you_use_energy_card_request.get_opponent_unique_id(),
            notify_to_opponent_you_use_energy_card_request.get_usage_energy_card_id(),
            notify_to_opponent_you_use_energy_card_request.get_unit_card_index(),
            notify_to_opponent_you_use_energy_card_request.get_energy_race(),
            notify_to_opponent_you_use_energy_card_request.get_energy_count()).await;

        NotifyToOpponentYouUseEnergyCardResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_to_opponent_you_use_energy_boost_support_card(&mut self, notify_to_opponent_you_use_energy_boost_card_request: NotifyToOpponentYouUseEnergyBoostCardRequest) -> NotifyToOpponentYouUseEnergyBoostCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_energy_boost_support_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_to_opponent_you_use_energy_boost_support_card(
            notify_to_opponent_you_use_energy_boost_card_request.get_opponent_unique_id(),
            notify_to_opponent_you_use_energy_boost_card_request.get_unit_card_index(),
            notify_to_opponent_you_use_energy_boost_card_request.get_usage_hand_card_id(),
            notify_to_opponent_you_use_energy_boost_card_request.get_boosting_energy_count(),
            notify_to_opponent_you_use_energy_boost_card_request.get_boosting_energy_card_id()).await;

        NotifyToOpponentYouUseEnergyBoostCardResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_to_opponent_you_use_instant_death_item_card(&mut self, notify_to_opponent_you_use_item_instant_death_request: NotifyToOpponentYouUseItemInstantDeathRequest) -> NotifyToOpponentYouUseItemCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_instant_death_item_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard
            .notify_to_opponent_you_use_instant_death_item_card(
                notify_to_opponent_you_use_item_instant_death_request.get_opponent_unique_id(),
                notify_to_opponent_you_use_item_instant_death_request.get_opponent_target_unit_index(),
                notify_to_opponent_you_use_item_instant_death_request.get_usage_item_card_id()).await;

        NotifyToOpponentYouUseItemCardResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_to_opponent_you_use_instant_death_alternatives_item_card(&mut self, notify_to_opponent_you_use_item_instant_death_alternatives_request: NotifyToOpponentYouUseItemInstantDeathAlternativesRequest) -> NotifyToOpponentYouUseItemInstantDeathAlternativesResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_instant_death_alternatives_item_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard
            .notify_to_opponent_you_use_instant_death_item_card_alternatives(
                notify_to_opponent_you_use_item_instant_death_alternatives_request.get_opponent_unique_id(),
                notify_to_opponent_you_use_item_instant_death_alternatives_request.get_opponent_target_unit_index(),
                notify_to_opponent_you_use_item_instant_death_alternatives_request.get_usage_item_card_id(),
                notify_to_opponent_you_use_item_instant_death_alternatives_request.get_alternatives_damage()).await;

        NotifyToOpponentYouUseItemInstantDeathAlternativesResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_to_opponent_you_use_draw_support_card(&mut self, notify_to_opponent_you_use_draw_support_card_request: NotifyToOpponentYouUseDrawSupportCardRequest) -> NotifyToOpponentYouUseDrawSupportCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_draw_support_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_to_opponent_you_use_draw_support_card(
            notify_to_opponent_you_use_draw_support_card_request.get_opponent_unique_id(),
            notify_to_opponent_you_use_draw_support_card_request.get_usage_hand_card_id(),
            notify_to_opponent_you_use_draw_support_card_request.get_draw_card_count()).await;

        NotifyToOpponentYouUseDrawSupportCardResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_opponent_you_use_search_support_card(&mut self, notify_opponent_you_use_search_support_request: NotifyOpponentYouUseSearchSupportRequest) -> NotifyOpponentYouUseSupportCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_opponent_you_use_search_support_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_opponent_you_use_search_support_card(
            notify_opponent_you_use_search_support_request.get_opponent_unique_id(),
            notify_opponent_you_use_search_support_request.get_usage_hand_card_id(),
            notify_opponent_you_use_search_support_request.get_found_card_count()).await;

        NotifyOpponentYouUseSupportCardResponse::new(notify_to_opponent_what_you_do_response)
    }

    async fn notify_opponent_you_use_field_energy_remove_support_card(&mut self, notify_to_opponent_you_use_field_energy_remove_support_card_request: NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest) -> NotifyOpponentYouUseSupportCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_opponent_you_use_field_energy_remove_support_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_opponent_you_use_remove_field_energy_support_card(
            notify_to_opponent_you_use_field_energy_remove_support_card_request.get_opponent_unique_id(),
            notify_to_opponent_you_use_field_energy_remove_support_card_request.get_usage_hand_card_id(),
            notify_to_opponent_you_use_field_energy_remove_support_card_request.get_field_energy_to_remove()).await;

        NotifyOpponentYouUseSupportCardResponse::new(notify_to_opponent_what_you_do_response)
    }

    // TODO: 갱신된 필드 에너지 정보를 추가해야 함
    async fn notify_opponent_you_use_item_field_energy_increase_item_card(&mut self, notify_opponent_you_use_item_field_energy_increase_request: NotifyOpponentYouUseItemFieldEnergyIncreaseRequest) -> NotifyToOpponentYouUseItemCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_opponent_you_use_item_field_energy_increase_item_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard.notify_opponent_you_use_field_energy_increase_item_card(
            notify_opponent_you_use_item_field_energy_increase_request.get_opponent_unique_id(),
            notify_opponent_you_use_item_field_energy_increase_request.get_usage_item_card_id(),
            notify_opponent_you_use_item_field_energy_increase_request.get_increased_field_energy()).await;

        NotifyToOpponentYouUseItemCardResponse::new(notify_to_opponent_what_you_do_response)
    }
    async fn notify_to_opponent_you_use_tool_card_to_enhance_attack_point(&mut self, notify_to_opponent_you_use_tool_card_to_enhance_attack_point_request: NotifyToOpponentYouUseToolCardToEnhanceAttackPointRequest) -> NotifyToOpponentYouUseToolCardToEnhanceAttackPointResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_tool_card_to_enhance_attack_point()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard
            .notify_to_opponent_you_use_tool_card_to_enhance_attack_point(
                notify_to_opponent_you_use_tool_card_to_enhance_attack_point_request.get_opponent_unique_id(),
                notify_to_opponent_you_use_tool_card_to_enhance_attack_point_request.get_unit_card_index(),
                notify_to_opponent_you_use_tool_card_to_enhance_attack_point_request.get_usage_tool_card_id()).await;

        NotifyToOpponentYouUseToolCardToEnhanceAttackPointResponse::new(notify_to_opponent_what_you_do_response)
    }
    async fn notify_to_opponent_you_use_catastrophic_damage_item_card(&mut self, notify_to_opponent_you_use_catastrophic_damage_item_card_request: NotifyToOpponentYouUseCatastrophicDamageItemCardRequest)
                                                                                                                                                -> NotifyToOpponentYouUseCatastrophicDamageItemCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_catastrophic_damage_item_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard
            .notify_to_opponent_you_use_catastrophic_damage_item_card(
                notify_to_opponent_you_use_catastrophic_damage_item_card_request.get_opponent_unique_id(),
                notify_to_opponent_you_use_catastrophic_damage_item_card_request.get_usage_hand_card_id(),
                notify_to_opponent_you_use_catastrophic_damage_item_card_request.get_damage_for_field_unit()).await;

        NotifyToOpponentYouUseCatastrophicDamageItemCardResponse::new(notify_to_opponent_what_you_do_response)

    }
    async fn notify_to_opponent_you_use_damage_main_character_item_card(&mut self, notify_to_opponent_you_use_damage_main_character_item_card_request: NotifyToOpponentYouUseDamageMainCharacterItemCardRequest)
                                                                                                                                                  -> NotifyToOpponentYouUseDamageMainCharacterItemCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_damage_main_character_item_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard
            .notify_to_opponent_you_use_damage_main_character_item_card(
                notify_to_opponent_you_use_damage_main_character_item_card_request.get_opponent_unique_id(),
                notify_to_opponent_you_use_damage_main_character_item_card_request.get_usage_hand_card_id(),
                notify_to_opponent_you_use_damage_main_character_item_card_request.get_damage_for_main_character()).await;

        NotifyToOpponentYouUseDamageMainCharacterItemCardResponse::new(notify_to_opponent_what_you_do_response)
    }
    async fn notify_to_opponent_you_use_destroy_deck_item_card(&mut self, notify_to_opponent_you_use_destroy_deck_item_card_request: NotifyToOpponentYouUseDestroyDeckItemCardRequest)
                                                                        -> NotifyToOpponentYouUseDestroyDeckItemCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_destroy_deck_card_item_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard
            .notify_to_opponent_you_use_destroy_deck_item_card(
                notify_to_opponent_you_use_destroy_deck_item_card_request.get_opponent_unique_id(),
                notify_to_opponent_you_use_destroy_deck_item_card_request.get_usage_hand_card_id(),
                notify_to_opponent_you_use_destroy_deck_item_card_request.get_will_be_lost_card()).await;

        NotifyToOpponentYouUseDestroyDeckItemCardResponse::new(notify_to_opponent_what_you_do_response)
    }
    async fn notify_to_opponent_you_use_field_unit_energy_removal_item_card(&mut self,notify_to_opponent_you_use_field_unit_energy_removal_item_card:NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardRequest)
                                                                                                                                                 ->NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardResponse {
        println!("NotifyPlayerActionServiceImpl: notify_to_opponent_you_use_field_unit_energy_removal_item_card()");

        let mut notify_player_action_repository_guard = self.notify_player_action_repository.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_repository_guard
            .notify_to_opponent_you_use_field_unit_energy_removal_item_card(
                notify_to_opponent_you_use_field_unit_energy_removal_item_card.get_opponent_unique_id(),
                notify_to_opponent_you_use_field_unit_energy_removal_item_card.get_usage_hand_card_id(),
                notify_to_opponent_you_use_field_unit_energy_removal_item_card.get_energy_quantity()).await;

        NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardResponse::new(notify_to_opponent_what_you_do_response)
    }
}