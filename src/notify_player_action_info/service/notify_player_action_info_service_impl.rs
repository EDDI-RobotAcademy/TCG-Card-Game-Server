use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::game_field_energy::repository::game_field_energy_repository_impl::GameFieldEnergyRepositoryImpl;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::notify_player_action_info::repository::notify_player_action_info_repository_impl::NotifyPlayerActionInfoRepositoryImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::request::notice_boost_energy_to_specific_unit_by_using_hand_card_request::{NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest};
use crate::notify_player_action_info::service::request::notice_draw_card_by_using_hand_card_request::NoticeDrawCardByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_remove_energy_of_specific_unit_by_using_hand_card_request::NoticeRemoveEnergyOfSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_remove_field_energy_by_using_hand_card_request::NoticeRemoveFieldEnergyByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_search_card_by_using_hand_card_request::NoticeSearchCardByUsingHandCardRequest;
use crate::notify_player_action_info::service::response::notice_boost_energy_to_specific_unit_by_using_hand_card_response::{NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse};
use crate::notify_player_action_info::service::response::notice_draw_card_by_using_hand_card_response::NoticeDrawCardByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_remove_energy_of_specific_unit_by_using_hand_card_response::NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_remove_field_energy_by_using_hand_card_response::NoticeRemoveFieldEnergyByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_search_card_by_using_hand_card_response::NoticeSearchCardByUsingHandCardResponse;

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
    async fn notice_boost_energy_to_specific_unit_by_using_hand_card(
        &mut self, notice_boost_energy_to_specific_unit_by_using_hand_card_request: NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest)
        -> NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_boost_energy_to_specific_unit_by_using_hand_card()");

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(
                &notice_boost_energy_to_specific_unit_by_using_hand_card_request.get_used_hand_card_id()).await;

        drop(card_kind_repository_guard);

        let mut game_field_unit_repository_guard=
            self.game_field_unit_repository.lock().await;

        let attached_energy_info =
            game_field_unit_repository_guard.acquire_energy_map_of_indexed_unit(
                notice_boost_energy_to_specific_unit_by_using_hand_card_request.get_account_unique_id(),
                notice_boost_energy_to_specific_unit_by_using_hand_card_request.get_unit_index()).to_attached_energy_info();

        drop(game_field_unit_repository_guard);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_boost_energy_to_specific_unit_by_using_hand_card(
                notice_boost_energy_to_specific_unit_by_using_hand_card_request.get_account_unique_id(),
                notice_boost_energy_to_specific_unit_by_using_hand_card_request.get_opponent_unique_id(),
                notice_boost_energy_to_specific_unit_by_using_hand_card_request.get_used_hand_card_id(),
                hand_card_kind_enum,
                notice_boost_energy_to_specific_unit_by_using_hand_card_request.get_found_energy_card_id_list().clone(),
                notice_boost_energy_to_specific_unit_by_using_hand_card_request.get_unit_index(),
                attached_energy_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse::new(response)
    }

    async fn notice_draw_card_by_using_hand_card(
        &mut self, notice_draw_card_by_using_hand_card_request: NoticeDrawCardByUsingHandCardRequest)
        -> NoticeDrawCardByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_draw_card_by_using_hand_card()");

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(
                &notice_draw_card_by_using_hand_card_request.get_used_hand_card_id()).await;

        drop(card_kind_repository_guard);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_draw_card_by_using_hand_card(
                notice_draw_card_by_using_hand_card_request.get_account_unique_id(),
                notice_draw_card_by_using_hand_card_request.get_opponent_unique_id(),
                notice_draw_card_by_using_hand_card_request.get_used_hand_card_id(),
                hand_card_kind_enum,
                notice_draw_card_by_using_hand_card_request.get_drawn_card_list().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeDrawCardByUsingHandCardResponse::new(response)
    }

    async fn notice_search_card_by_using_hand_card(
        &mut self, notice_search_card_by_using_hand_card_request: NoticeSearchCardByUsingHandCardRequest)
        -> NoticeSearchCardByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_search_card_by_using_hand_card()");

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(
                &notice_search_card_by_using_hand_card_request.get_used_hand_card_id()).await;

        drop(card_kind_repository_guard);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_search_card_by_using_hand_card(
                notice_search_card_by_using_hand_card_request.get_account_unique_id(),
                notice_search_card_by_using_hand_card_request.get_opponent_unique_id(),
                notice_search_card_by_using_hand_card_request.get_used_hand_card_id(),
                hand_card_kind_enum,
                notice_search_card_by_using_hand_card_request.get_found_card_list().clone()).await;

        drop(notify_player_action_info_repository_guard);

        NoticeSearchCardByUsingHandCardResponse::new(response)
    }

    async fn notice_remove_field_energy_by_using_hand_card(
        &mut self, notice_remove_field_energy_by_using_hand_card_request: NoticeRemoveFieldEnergyByUsingHandCardRequest)
        -> NoticeRemoveFieldEnergyByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_remove_energy_of_specific_unit_by_using_hand_card()");

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(
                &notice_remove_field_energy_by_using_hand_card_request.get_used_hand_card_id()).await;

        drop(card_kind_repository_guard);

        let mut game_field_energy_repository_guard =
            self.game_field_energy_repository.lock().await;

        let remaining_opponent_field_energy_count =
            game_field_energy_repository_guard.get_game_field_energy_map()
                .get_mut(&notice_remove_field_energy_by_using_hand_card_request.get_opponent_unique_id()).unwrap()
                .get_energy_count();

        drop(game_field_energy_repository_guard);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_remove_field_energy_by_using_hand_card(
                notice_remove_field_energy_by_using_hand_card_request.get_account_unique_id(),
                notice_remove_field_energy_by_using_hand_card_request.get_opponent_unique_id(),
                notice_remove_field_energy_by_using_hand_card_request.get_used_hand_card_id(),
                hand_card_kind_enum,
                remaining_opponent_field_energy_count).await;

        drop(notify_player_action_info_repository_guard);

        NoticeRemoveFieldEnergyByUsingHandCardResponse::new(response)
    }

    async fn notice_remove_energy_of_specific_unit_by_using_hand_card(
        &mut self, notice_remove_energy_of_specific_unit_by_using_hand_card_request: NoticeRemoveEnergyOfSpecificUnitByUsingHandCardRequest)
        -> NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse {

        println!("NotifyPlayerActionInfoServiceImpl: notice_remove_energy_of_specific_unit_by_using_hand_card()");

        let mut card_kind_repository_guard =
            self.card_kind_repository.lock().await;

        let hand_card_kind_enum =
            card_kind_repository_guard.get_card_kind(
                &notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_used_hand_card_id()).await;

        drop(card_kind_repository_guard);

        let mut game_field_unit_repository_guard=
            self.game_field_unit_repository.lock().await;

        let attached_energy_info =
            game_field_unit_repository_guard.acquire_energy_map_of_indexed_unit(
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_account_unique_id(),
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_unit_index()).to_attached_energy_info();

        drop(game_field_unit_repository_guard);

        let mut notify_player_action_info_repository_guard =
            self.notify_player_action_info_repository.lock().await;

        let response =
            notify_player_action_info_repository_guard.notify_player_remove_energy_of_specific_unit_by_using_hand_card(
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_account_unique_id(),
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_opponent_unique_id(),
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_used_hand_card_id(),
                hand_card_kind_enum,
                notice_remove_energy_of_specific_unit_by_using_hand_card_request.get_unit_index(),
                attached_energy_info).await;

        drop(notify_player_action_info_repository_guard);

        NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse::new(response)
    }
}