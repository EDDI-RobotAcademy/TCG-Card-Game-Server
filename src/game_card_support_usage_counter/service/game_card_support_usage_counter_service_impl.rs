use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_support_usage_counter::repository::game_card_support_usage_counter_repository::GameCardSupportUsageCounterRepository;
use crate::game_card_support_usage_counter::repository::game_card_support_usage_counter_repository_impl::GameCardSupportUsageCounterRepositoryImpl;
use crate::game_card_support_usage_counter::service::game_card_support_usage_counter_service::GameCardSupportUsageCounterService;
use crate::game_card_support_usage_counter::service::request::check_support_card_usage_count_request::CheckSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::reset_support_card_usage_count_request::ResetSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::update_support_card_usage_count_request::UpdateSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::response::check_support_card_usage_count_response::CheckSupportCardUsageCountResponse;
use crate::game_card_support_usage_counter::service::response::reset_support_card_usage_count_response::ResetSupportCardUsageCountResponse;
use crate::game_card_support_usage_counter::service::response::update_support_card_usage_count_response::UpdateSupportCardUsageCountResponse;

pub struct GameCardSupportUsageCounterServiceImpl {
    game_card_support_usage_counter_repository: Arc<AsyncMutex<GameCardSupportUsageCounterRepositoryImpl>>
}

impl GameCardSupportUsageCounterServiceImpl {
    pub fn new(game_card_support_usage_counter_repository: Arc<AsyncMutex<GameCardSupportUsageCounterRepositoryImpl>>) -> Self {
        GameCardSupportUsageCounterServiceImpl {
            game_card_support_usage_counter_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportUsageCounterServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportUsageCounterServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportUsageCounterServiceImpl::new(
                            GameCardSupportUsageCounterRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardSupportUsageCounterService for GameCardSupportUsageCounterServiceImpl {
    async fn reset_support_card_usage_count(
        &mut self, reset_support_card_usage_count_request: ResetSupportCardUsageCountRequest)
        -> ResetSupportCardUsageCountResponse {

        println!("GameCardSupportUsageCounterServiceImpl: reset_support_card_usage_count()");

        let mut game_card_support_usage_counter_repository =
            self.game_card_support_usage_counter_repository.lock().await;

        let result = game_card_support_usage_counter_repository.reset_support_card_usage_counter(
            reset_support_card_usage_count_request.get_account_unique_id());

        ResetSupportCardUsageCountResponse::new(result)
    }

    async fn check_support_card_usage_count(
        &mut self, check_support_card_usage_count_request: CheckSupportCardUsageCountRequest)
        -> CheckSupportCardUsageCountResponse {

        println!("GameCardSupportUsageCounterServiceImpl: check_support_card_usage_count()");

        let mut game_card_support_usage_counter_repository =
            self.game_card_support_usage_counter_repository.lock().await;

        let used_count = game_card_support_usage_counter_repository.check_support_card_usage_counter(
            check_support_card_usage_count_request.get_account_unique_id());

        CheckSupportCardUsageCountResponse::new(used_count)
    }

    async fn update_support_card_usage_count(
        &mut self, update_support_card_usage_count_request: UpdateSupportCardUsageCountRequest)
        -> UpdateSupportCardUsageCountResponse {

        println!("GameCardSupportUsageCounterServiceImpl: update_support_card_usage_count()");

        let mut game_card_support_usage_counter_repository =
            self.game_card_support_usage_counter_repository.lock().await;

        let result = game_card_support_usage_counter_repository.update_support_card_usage_counter(
            update_support_card_usage_count_request.get_account_unique_id());

        UpdateSupportCardUsageCountResponse::new(result)
    }
}