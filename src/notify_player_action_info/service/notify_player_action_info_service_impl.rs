use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::game_field_energy::repository::game_field_energy_repository_impl::GameFieldEnergyRepositoryImpl;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::ui_data_generator::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::field_unit_death_info::{FieldUnitDeathInfo};
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::notify_player_action_info::repository::notify_player_action_info_repository_impl::NotifyPlayerActionInfoRepositoryImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::request::notice_add_field_energy_request::NoticeAddFieldEnergyRequest;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_every_opponent_unit_request::{NoticeApplyDamageToEveryOpponentUnitRequest};
use crate::notify_player_action_info::service::request::notice_apply_damage_to_multiple_opponent_unit_request::NoticeApplyDamageToMultipleOpponentUnitRequest;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_opponent_main_character_request::NoticeApplyDamageToOpponentMainCharacterRequest;
use crate::notify_player_action_info::service::request::notice_boost_energy_to_specific_unit_request::{NoticeBoostEnergyToSpecificUnitRequest};
use crate::notify_player_action_info::service::request::notice_apply_damage_to_specific_opponent_unit_request::{NoticeApplyDamageToSpecificOpponentUnitRequest};
use crate::notify_player_action_info::service::request::notice_attach_energy_to_specific_unit_request::{NoticeAttachEnergyToSpecificUnitRequest};
use crate::notify_player_action_info::service::request::notice_draw_card_request::{NoticeDrawCardRequest};
use crate::notify_player_action_info::service::request::notice_instant_death_of_specific_opponent_unit_request::{NoticeInstantDeathOfSpecificOpponentUnitRequest};
use crate::notify_player_action_info::service::request::notice_instant_death_of_specific_unit_request::NoticeInstantDeathOfSpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_lost_deck_card_of_opponent_request::NoticeLostDeckCardOfOpponentRequest;
use crate::notify_player_action_info::service::request::notice_remove_energy_of_specific_opponent_unit_request::{NoticeRemoveEnergyOfSpecificOpponentUnitRequest};
use crate::notify_player_action_info::service::request::notice_remove_field_energy_of_opponent_request::{NoticeRemoveFieldEnergyOfOpponentRequest};
use crate::notify_player_action_info::service::request::notice_search_card_request::{NoticeSearchCardRequest};
use crate::notify_player_action_info::service::request::notice_use_catastrophic_damage_item_card_request::NoticeUseCatastrophicDamageItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_draw_support_card_request::NoticeUseDrawSupportCardRequest;
use crate::notify_player_action_info::service::request::notice_use_energy_boost_support_card_to_my_specific_unit_request::NoticeUseEnergyBoostSupportCardToSpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_increase_item_card_request::NoticeUseFieldEnergyIncreaseItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_remove_support_card_request::NoticeUseFieldEnergyRemoveSupportCardRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_to_my_specific_unit_request::NoticeUseFieldEnergyToMySpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_general_energy_card_to_my_specific_unit_request::NoticeUseGeneralEnergyCardToMySpecificUnitRequest;
use crate::notify_player_action_info::service::request::notice_use_hand_card_request::NoticeUseHandCardRequest;
use crate::notify_player_action_info::service::request::notice_use_instant_unit_death_item_card_request::NoticeUseInstantUnitDeathItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_multiple_unit_damage_item_card_request::NoticeUseMultipleUnitDamageItemCardRequest;
use crate::notify_player_action_info::service::request::notice_use_search_deck_support_card_request::{NoticeUseSearchDeckSupportCardRequest};
use crate::notify_player_action_info::service::request::notice_use_unit_energy_remove_item_card_request::NoticeUseUnitEnergyRemoveItemCardRequest;
use crate::notify_player_action_info::service::response::notice_add_field_energy_response::NoticeAddFieldEnergyResponse;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_every_opponent_unit_response::{NoticeApplyDamageToEveryOpponentUnitResponse};
use crate::notify_player_action_info::service::response::notice_apply_damage_to_multiple_opponent_unit_response::NoticeApplyDamageToMultipleOpponentUnitResponse;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_opponent_main_character_response::NoticeApplyDamageToOpponentMainCharacterResponse;
use crate::notify_player_action_info::service::response::notice_boost_energy_to_specific_unit_response::{NoticeBoostEnergyToSpecificUnitResponse};
use crate::notify_player_action_info::service::response::notice_apply_damage_to_specific_opponent_unit_response::{NoticeApplyDamageToSpecificOpponentUnitResponse};
use crate::notify_player_action_info::service::response::notice_attach_energy_to_specific_unit_response::{NoticeAttachEnergyToSpecificUnitResponse};
use crate::notify_player_action_info::service::response::notice_draw_card_response::{NoticeDrawCardResponse};
use crate::notify_player_action_info::service::response::notice_instant_death_of_specific_opponent_unit_response::{NoticeInstantDeathOfSpecificOpponentUnitResponse};
use crate::notify_player_action_info::service::response::notice_instant_death_of_specific_unit_response::NoticeInstantDeathOfSpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_lost_deck_card_opponent_response::NoticeLostDeckCardOfOpponentResponse;
use crate::notify_player_action_info::service::response::notice_remove_energy_of_specific_opponent_unit_response::{NoticeRemoveEnergyOfSpecificOpponentUnitResponse};
use crate::notify_player_action_info::service::response::notice_remove_field_energy_of_opponent_response::{NoticeRemoveFieldEnergyOfOpponentResponse};
use crate::notify_player_action_info::service::response::notice_search_card_response::{NoticeSearchCardResponse};
use crate::notify_player_action_info::service::response::notice_use_catastrophic_damage_item_card_response::NoticeUseCatastrophicDamageItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_draw_support_card_response::NoticeUseDrawSupportCardResponse;
use crate::notify_player_action_info::service::response::notice_use_energy_boost_support_card_to_my_specific_unit_response::{NoticeUseEnergyBoostSupportCardToSpecificUnitResponse};
use crate::notify_player_action_info::service::response::notice_use_field_energy_increase_item_card_response::NoticeUseFieldEnergyIncreaseItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_field_energy_remove_support_card_response::NoticeUseFieldEnergyRemoveSupportCardResponse;
use crate::notify_player_action_info::service::response::notice_use_field_energy_to_my_specific_unit_response::NoticeUseFieldEnergyToMySpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_general_energy_card_to_my_specific_unit_response::NoticeUseGeneralEnergyCardToMySpecificUnitResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;
use crate::notify_player_action_info::service::response::notice_use_instant_unit_death_item_card_response::NoticeUseInstantUnitDeathItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_multiple_unit_damage_item_card_response::NoticeUseMultipleUnitDamageItemCardResponse;
use crate::notify_player_action_info::service::response::notice_use_search_deck_support_card_response::{NoticeUseSearchDeckSupportCardResponse};
use crate::notify_player_action_info::service::response::notice_use_unit_energy_remove_item_card_response::NoticeUseUnitEnergyRemoveItemCardResponse;

pub struct NotifyPlayerActionInfoServiceImpl {
    notify_player_action_info_repository: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
    card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
}

impl NotifyPlayerActionInfoServiceImpl {
    pub fn new(
        notify_player_action_info_repository: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
        card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
        game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
        game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
    ) -> Self {

        NotifyPlayerActionInfoServiceImpl {
            notify_player_action_info_repository,
            card_kind_repository,
            game_field_unit_repository,
            game_field_energy_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionInfoServiceImpl::new(
                            NotifyPlayerActionInfoRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance(),
                            GameFieldEnergyRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl NotifyPlayerActionInfoService for NotifyPlayerActionInfoServiceImpl {
    async fn notice_use_hand_card(
        &mut self, notice_use_hand_card_request: NoticeUseHandCardRequest)
        -> NoticeUseHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_use_hand_card()");

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(
                &notice_use_hand_card_request.get_used_hand_card_id()).await;

        drop(card_kind_repository_guard);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_use_hand_card(
                notice_use_hand_card_request.get_opponent_unique_id(),
                notice_use_hand_card_request.get_used_hand_card_id(),
                hand_card_kind_enum).await;

        drop(notify_player_action_info_repository_guard);

        NoticeUseHandCardResponse::new(response)
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

    async fn notice_boost_energy_to_specific_unit(
        &mut self,
        notice_boost_energy_to_specific_unit_request: NoticeBoostEnergyToSpecificUnitRequest)
        -> NoticeBoostEnergyToSpecificUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_boost_energy_to_specific_unit()");

        let mut field_unit_energy_map = HashMap::new();
        field_unit_energy_map.insert(
            notice_boost_energy_to_specific_unit_request.get_unit_index(),
            notice_boost_energy_to_specific_unit_request.get_updated_attached_energy_map().to_attached_energy_info());

        let field_unit_energy_info = FieldUnitEnergyInfo::new(field_unit_energy_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let player_deck_card_use_list_info =
            notify_player_action_info_repository_guard.notify_player_use_deck_card_list(
                notice_boost_energy_to_specific_unit_request.get_opponent_unique_id(),
                notice_boost_energy_to_specific_unit_request.get_found_energy_card_id_list().clone()).await;
        let player_field_unit_energy_info =
            notify_player_action_info_repository_guard.notify_player_energy_of_unit(
                notice_boost_energy_to_specific_unit_request.get_opponent_unique_id(),
                field_unit_energy_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeBoostEnergyToSpecificUnitResponse::new(
            player_deck_card_use_list_info, player_field_unit_energy_info)
    }

    async fn notice_draw_card(
        &mut self,
        notice_draw_card_request: NoticeDrawCardRequest)
        -> NoticeDrawCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_draw_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let player_drawn_card_list_info =
            notify_player_action_info_repository_guard.notify_player_draw_card(
                notice_draw_card_request.get_opponent_unique_id(),
                notice_draw_card_request.get_drawn_card_list().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeDrawCardResponse::new(player_drawn_card_list_info)
    }

    async fn notice_search_card(
        &mut self,
        notice_search_card_request: NoticeSearchCardRequest)
        -> NoticeSearchCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_search_card()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_search_card(
                notice_search_card_request.get_opponent_unique_id(),
                notice_search_card_request.get_found_card_list().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeSearchCardResponse::new(response)
    }

    async fn notice_add_field_energy(
        &mut self, notice_add_field_energy_request: NoticeAddFieldEnergyRequest)
        -> NoticeAddFieldEnergyResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_add_field_energy()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_field_energy(
                notice_add_field_energy_request.get_opponent_unique_id(),
                notice_add_field_energy_request.get_remaining_field_energy()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeAddFieldEnergyResponse::new(response)
    }

    async fn notice_remove_field_energy_of_opponent(
        &mut self,
        notice_remove_field_energy_of_opponent_request: NoticeRemoveFieldEnergyOfOpponentRequest)
        -> NoticeRemoveFieldEnergyOfOpponentResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_remove_field_energy_of_opponent()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_opponent_field_energy(
                notice_remove_field_energy_of_opponent_request.get_opponent_unique_id(),
                notice_remove_field_energy_of_opponent_request.get_remaining_field_energy()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeRemoveFieldEnergyOfOpponentResponse::new(response)
    }

    async fn notice_remove_energy_of_specific_opponent_unit(
        &mut self,
        notice_remove_energy_of_specific_opponent_unit_request: NoticeRemoveEnergyOfSpecificOpponentUnitRequest)
        -> NoticeRemoveEnergyOfSpecificOpponentUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_remove_energy_of_specific_opponent_unit()");

        let mut field_unit_energy_map = HashMap::new();
        field_unit_energy_map.insert(
            notice_remove_energy_of_specific_opponent_unit_request.get_opponent_unit_index(),
            notice_remove_energy_of_specific_opponent_unit_request
                .get_updated_opponent_unit_energy_map().to_attached_energy_info());

        let field_unit_energy_info = FieldUnitEnergyInfo::new(field_unit_energy_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_energy_of_specific_opponent_unit(
                notice_remove_energy_of_specific_opponent_unit_request.get_opponent_unique_id(),
                field_unit_energy_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeRemoveEnergyOfSpecificOpponentUnitResponse::new(response)
    }

    async fn notice_apply_damage_to_specific_opponent_unit(
        &mut self,
        notice_apply_damage_to_specific_opponent_unit_request: NoticeApplyDamageToSpecificOpponentUnitRequest)
        -> NoticeApplyDamageToSpecificOpponentUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_apply_damage_to_specific_opponent_unit()");

        let mut field_unit_damage_map = HashMap::new();
        field_unit_damage_map.insert(
            notice_apply_damage_to_specific_opponent_unit_request.get_opponent_unit_index(),
            notice_apply_damage_to_specific_opponent_unit_request.get_damage());

        let field_unit_damage_info =
            FieldUnitDamageInfo::new(field_unit_damage_map);

        let mut field_unit_health_point_map = HashMap::new();
        field_unit_health_point_map.insert(
            notice_apply_damage_to_specific_opponent_unit_request.get_opponent_unit_index(),
            notice_apply_damage_to_specific_opponent_unit_request.get_updated_health_point());

        let field_unit_health_info =
            FieldUnitHealthPointInfo::new(field_unit_health_point_map);

        let mut dead_unit_index_list = Vec::new();
        if notice_apply_damage_to_specific_opponent_unit_request.get_dead_unit_index() != -1 {
            dead_unit_index_list.push(
                notice_apply_damage_to_specific_opponent_unit_request.get_dead_unit_index())
        }

        let field_unit_death_info =
            FieldUnitDeathInfo::new(dead_unit_index_list);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_apply_damage_to_opponent_unit(
                notice_apply_damage_to_specific_opponent_unit_request.get_opponent_unique_id(),
                field_unit_damage_info,
                field_unit_health_info,
                field_unit_death_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeApplyDamageToSpecificOpponentUnitResponse::new(response.0, response.1, response.2)
    }

    async fn notice_apply_damage_to_multiple_opponent_unit(
        &mut self, notice_apply_damage_to_multiple_opponent_unit_request: NoticeApplyDamageToMultipleOpponentUnitRequest)
        -> NoticeApplyDamageToMultipleOpponentUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_apply_damage_to_multiple_opponent_unit()");

        let target_unit_index =
            notice_apply_damage_to_multiple_opponent_unit_request.get_target_unit_index_list();
        let damage = notice_apply_damage_to_multiple_opponent_unit_request.get_damage();
        let updated_health_point_list =
            notice_apply_damage_to_multiple_opponent_unit_request.get_updated_health_point_list();

        let mut field_unit_damage_map = HashMap::new();
        let mut field_unit_health_point_map = HashMap::new();

        for (index, target_unit_index) in target_unit_index.iter().enumerate() {
            field_unit_damage_map.insert(*target_unit_index, damage);
            if let Some(&health_point) = updated_health_point_list.get(index) {
                field_unit_health_point_map.insert(*target_unit_index, health_point);
            }
        }

        let dead_unit_index_list =
            notice_apply_damage_to_multiple_opponent_unit_request.get_dead_unit_index_list().clone();

        let field_unit_damage_info =
            FieldUnitDamageInfo::new(field_unit_damage_map);

        let field_unit_health_info =
            FieldUnitHealthPointInfo::new(field_unit_health_point_map);

        let field_unit_death_info =
            FieldUnitDeathInfo::new(dead_unit_index_list);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_apply_damage_to_opponent_unit(
                notice_apply_damage_to_multiple_opponent_unit_request.get_opponent_unique_id(),
                field_unit_damage_info,
                field_unit_health_info,
                field_unit_death_info).await;

        NoticeApplyDamageToMultipleOpponentUnitResponse::new(response.0, response.1, response.2)
    }

    async fn notice_apply_damage_to_every_opponent_unit(
        &mut self,
        notice_apply_damage_to_every_opponent_unit_request: NoticeApplyDamageToEveryOpponentUnitRequest)
        -> NoticeApplyDamageToEveryOpponentUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_apply_damage_to_every_opponent_unit()");

        let mut field_unit_damage_map = HashMap::new();
        let mut field_unit_health_point_map = HashMap::new();

        let damage = notice_apply_damage_to_every_opponent_unit_request.get_damage();
        let updated_health_point_list =
            notice_apply_damage_to_every_opponent_unit_request.get_updated_health_point_list();

        // TODO: 나중에 대미지를 받지 않은 유닛이 있는 경우에 대해 처리가 필요함
        for (unit_index, updated_health_point) in updated_health_point_list.iter().enumerate() {
            field_unit_damage_map.insert(unit_index as i32, damage);
            field_unit_health_point_map.insert(unit_index as i32, *updated_health_point);
        }

        let dead_unit_index_list =
            notice_apply_damage_to_every_opponent_unit_request.get_dead_unit_index_list().clone();

        let field_unit_damage_info =
            FieldUnitDamageInfo::new(field_unit_damage_map);

        let field_unit_health_info =
            FieldUnitHealthPointInfo::new(field_unit_health_point_map);

        let field_unit_death_info =
            FieldUnitDeathInfo::new(dead_unit_index_list);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_apply_damage_to_opponent_unit(
                notice_apply_damage_to_every_opponent_unit_request.get_opponent_unique_id(),
                field_unit_damage_info,
                field_unit_health_info,
                field_unit_death_info).await;

        NoticeApplyDamageToEveryOpponentUnitResponse::new(response.0, response.1, response.2)
    }

    async fn notice_apply_damage_to_opponent_main_character(
        &mut self, notice_apply_damage_to_opponent_main_character_request: NoticeApplyDamageToOpponentMainCharacterRequest)
        -> NoticeApplyDamageToOpponentMainCharacterResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_apply_damage_to_opponent_main_character()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_apply_damage_to_opponent_main_character(
                notice_apply_damage_to_opponent_main_character_request.get_opponent_unique_id(),
                notice_apply_damage_to_opponent_main_character_request.get_damage(),
                notice_apply_damage_to_opponent_main_character_request.get_updated_health_point(),
                notice_apply_damage_to_opponent_main_character_request.get_opponent_survival_status().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeApplyDamageToOpponentMainCharacterResponse::new(response.0, response.1, response.2)
    }

    async fn notice_lost_deck_card_of_opponent(
        &mut self, notice_lost_deck_card_of_opponent_request: NoticeLostDeckCardOfOpponentRequest)
        -> NoticeLostDeckCardOfOpponentResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_lost_deck_card_of_opponent()");

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_lost_deck_card(
                notice_lost_deck_card_of_opponent_request.get_opponent_unique_id(),
                notice_lost_deck_card_of_opponent_request.get_lost_deck_card_list().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeLostDeckCardOfOpponentResponse::new(response)
    }

    async fn notice_attach_energy_to_specific_unit(
        &mut self,
        notice_attach_energy_to_specific_unit_request: NoticeAttachEnergyToSpecificUnitRequest)
        -> NoticeAttachEnergyToSpecificUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_attach_energy_to_specific_unit()");

        let mut field_unit_energy_map = HashMap::new();
        field_unit_energy_map.insert(
            notice_attach_energy_to_specific_unit_request.get_unit_index(),
            notice_attach_energy_to_specific_unit_request
                .get_updated_unit_energy_map().to_attached_energy_info());

        let field_unit_energy_info =
            FieldUnitEnergyInfo::new(field_unit_energy_map);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_attach_energy_to_specific_unit(
                notice_attach_energy_to_specific_unit_request.get_opponent_unique_id(),
                field_unit_energy_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeAttachEnergyToSpecificUnitResponse::new(response)
    }

    async fn notice_instant_death_of_specific_unit(
        &mut self, notice_instant_death_of_specific_unit_request: NoticeInstantDeathOfSpecificUnitRequest)
        -> NoticeInstantDeathOfSpecificUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_instant_death_of_multiple_opponent_unit()");

        let mut dead_unit_index_list = Vec::new();
        if notice_instant_death_of_specific_unit_request.get_dead_unit_index() != -1 {
            dead_unit_index_list.push(
                notice_instant_death_of_specific_unit_request.get_dead_unit_index())
        }

        let field_unit_death_info =
            FieldUnitDeathInfo::new(dead_unit_index_list);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_death_of_unit(
                notice_instant_death_of_specific_unit_request.get_opponent_unique_id(),
                field_unit_death_info).await;

        drop(notify_player_action_info_repository_guard);


        NoticeInstantDeathOfSpecificUnitResponse::new(response)
    }

    async fn notice_instant_death_of_specific_opponent_unit(
        &mut self,
        notice_instant_death_of_specific_opponent_unit_request: NoticeInstantDeathOfSpecificOpponentUnitRequest)
        -> NoticeInstantDeathOfSpecificOpponentUnitResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_instant_death_of_specific_opponent_unit()");

        let mut dead_unit_index_list = Vec::new();
        if notice_instant_death_of_specific_opponent_unit_request.get_dead_unit_index() != -1 {
            dead_unit_index_list.push(
                notice_instant_death_of_specific_opponent_unit_request.get_dead_unit_index())
        }

        let field_unit_death_info =
            FieldUnitDeathInfo::new(dead_unit_index_list);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_death_of_opponent_unit(
                notice_instant_death_of_specific_opponent_unit_request.get_opponent_unique_id(),
                field_unit_death_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeInstantDeathOfSpecificOpponentUnitResponse::new(response)
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
}