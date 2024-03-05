use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository::BattleReadyAccountHashRepository;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository_impl::BattleReadyAccountHashRepositoryImpl;
use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;

use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::request::battle_match_request::BattleMatchRequest;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::battle_room::service::request::what_is_the_room_number_request::WhatIsTheRoomNumberRequest;
use crate::battle_room::service::response::battle_match_response::BattleMatchResponse;
use crate::battle_room::service::response::find_opponent_by_account_id_response::FindOpponentByAccountIdResponse;
use crate::battle_room::service::response::what_is_the_room_number_response::WhatIsTheRoomNumberResponse;
use crate::battle_wait_queue::repository::battle_wait_queue_repository::BattleWaitQueueRepository;
use crate::battle_wait_queue::repository::battle_wait_queue_repository_impl::BattleWaitQueueRepositoryImpl;
use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct BattleRoomServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
    battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
    battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
}

impl BattleRoomServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
               battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
               battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
            ) -> Self {

        BattleRoomServiceImpl {
            redis_in_memory_repository,
            battle_room_repository,
            battle_ready_account_hash_repository,
            battle_wait_queue_repository,
            match_waiting_timer_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleRoomServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleRoomServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleRoomServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance(),
                            BattleRoomRepositoryImpl::get_instance(),
                            BattleReadyAccountHashRepositoryImpl::get_instance(),
                            BattleWaitQueueRepositoryImpl::get_instance(),
                            MatchWaitingTimerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn get_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
        account_unique_id
    }
}

#[async_trait]
impl BattleRoomService for BattleRoomServiceImpl {
    async fn what_is_the_room_number(&self, what_is_the_room_number_request: WhatIsTheRoomNumberRequest) -> WhatIsTheRoomNumberResponse {
        println!("BattleRoomServiceImpl: what_is_the_room_number()");

        let session_id = what_is_the_room_number_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let battle_room_repository_guard = self.battle_room_repository.lock().await;
        let response = battle_room_repository_guard.what_is_the_room_number(account_unique_id).await;

        return WhatIsTheRoomNumberResponse::new(response.unwrap());
    }

    async fn find_opponent_by_account_unique_id(&self, find_opponent_by_account_id_request: FindOpponentByAccountIdRequest) -> FindOpponentByAccountIdResponse {
        println!("BattleRoomServiceImpl: find_opponent_by_account_unique_id()");

        let battle_room_repository_guard = self.battle_room_repository.lock().await;
        let maybe_opponent_unique_id = battle_room_repository_guard.find_opponent_unique_id(
            find_opponent_by_account_id_request.get_account_unique_id()).await;

        return FindOpponentByAccountIdResponse::new(maybe_opponent_unique_id.unwrap_or(-1))
    }
}