use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::notify_player_action_info::repository::notify_player_action_info_repository_impl::NotifyPlayerActionInfoRepositoryImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::request::notice_draw_card_by_using_hand_card_request::NoticeDrawCardByUsingHandCardRequest;
use crate::notify_player_action_info::service::response::notice_draw_card_by_using_hand_card_response::NoticeDrawCardByUsingHandCardResponse;

pub struct NotifyPlayerActionInfoServiceImpl {
    notify_player_action_info_repository: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
    card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>
}

impl NotifyPlayerActionInfoServiceImpl {
    pub fn new(
        notify_player_action_info_repository: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>>,
        card_kind_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>
    ) -> Self {

        NotifyPlayerActionInfoServiceImpl {
            notify_player_action_info_repository,
            card_kind_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionInfoServiceImpl::new(
                            NotifyPlayerActionInfoRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl NotifyPlayerActionInfoService for NotifyPlayerActionInfoServiceImpl {
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
            notify_player_action_info_repository_guard.notify_player_draw_card_with_using_hand_card(
                notice_draw_card_by_using_hand_card_request.get_account_unique_id(),
                notice_draw_card_by_using_hand_card_request.get_opponent_unique_id(),
                notice_draw_card_by_using_hand_card_request.get_used_hand_card_id(),
                hand_card_kind_enum,
                notice_draw_card_by_using_hand_card_request.get_drawn_card_list().clone()).await;

        NoticeDrawCardByUsingHandCardResponse::new(response)
    }
}