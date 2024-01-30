use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository::BattleReadyAccountHashRepository;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository_impl::BattleReadyAccountHashRepositoryImpl;

use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::request::battle_match_request::BattleMatchRequest;
use crate::battle_room::service::response::battle_match_response::BattleMatchResponse;
use crate::battle_wait_queue::repository::battle_wait_queue_repository::BattleWaitQueueRepository;
use crate::battle_wait_queue::repository::battle_wait_queue_repository_impl::BattleWaitQueueRepositoryImpl;
use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct BattleRoomServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
    battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
}

impl BattleRoomServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
               battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
            ) -> Self {

        BattleRoomServiceImpl {
            redis_in_memory_repository,
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
                            BattleReadyAccountHashRepositoryImpl::get_instance(),
                            BattleWaitQueueRepositoryImpl::get_instance(),
                            MatchWaitingTimerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleRoomService for BattleRoomServiceImpl {

    // async fn enqueue_player_id_to_wait_queue(&self, battle_match_request: BattleMatchRequest) -> BattleMatchResponse {
    //     println!("BattleRoomServiceImpl: enqueue_player_id_to_wait_queue()");
    //
    //     let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
    //     let account_unique_id_option_string = redis_in_memory_repository.get(battle_match_request.get_session_id()).await;
    //     let account_unique_id_string = account_unique_id_option_string.unwrap();
    //     let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
    //
    //     let battle_wait_queue_repository = self.battle_wait_queue_repository.lock().await;
    //
    //     let mut match_waiting_timer_repository = self.match_waiting_timer_repository.lock().await;
    //     match_waiting_timer_repository.set_match_waiting_timer(account_unique_id).await;
    //
    //     let mut battle_ready_monitor_repository = self.battle_ready_account_hash_repository.lock().await;
    //     battle_ready_monitor_repository.save_battle_ready_account_hash(account_unique_id, BattleReadyAccountHashStatus::WAIT).await;
    //
    //     let response = battle_wait_queue_repository.enqueue_player_id_for_wait(account_unique_id).await;
    //
    //     if response.is_ok() {
    //         return BattleMatchResponse::new(true)
    //     }
    //
    //     return BattleMatchResponse::new(false)
    // }
}