use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account::service::response::account_register_response::AccountRegisterResponse;
use crate::battle_room::repository::battle_room_ready_queue_repository::BattleRoomReadyQueueRepository;

use crate::battle_room::repository::battle_room_ready_queue_repository_impl::BattleRoomReadyQueueRepositoryImpl;
use crate::battle_room::repository::battle_room_wait_queue_repository::BattleRoomWaitQueueRepository;
use crate::battle_room::repository::battle_room_wait_queue_repository_impl::BattleRoomWaitQueueRepositoryImpl;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::request::battle_match_request::BattleMatchRequest;
use crate::battle_room::service::response::battle_match_response::BattleMatchResponse;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct BattleRoomServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    battle_room_ready_queue_repository: Arc<AsyncMutex<BattleRoomReadyQueueRepositoryImpl>>,
    battle_room_wait_queue_repository: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>>,
}

impl BattleRoomServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               battle_room_ready_queue_repository: Arc<AsyncMutex<BattleRoomReadyQueueRepositoryImpl>>,
               battle_room_wait_queue_repository: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>>
            ) -> Self {

        BattleRoomServiceImpl {
            redis_in_memory_repository,
            battle_room_ready_queue_repository,
            battle_room_wait_queue_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleRoomServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleRoomServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleRoomServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance(),
                            BattleRoomReadyQueueRepositoryImpl::get_instance(),
                            BattleRoomWaitQueueRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleRoomService for BattleRoomServiceImpl {

    async fn enqueue_player_id_to_wait_queue(&self, battle_match_request: BattleMatchRequest) -> BattleMatchResponse {
        println!("BattleRoomServiceImpl: enqueue_player_id_to_wait_queue()");

        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(battle_match_request.get_session_id()).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");

        let battle_room_wait_queue_repository = self.battle_room_wait_queue_repository.lock().await;
        let response = battle_room_wait_queue_repository.enqueue_player_id_for_wait(account_unique_id).await;

        if response.is_ok() {
            return BattleMatchResponse::new(true)
        }

        return BattleMatchResponse::new(false)
    }

    async fn enqueue_player_id_to_ready_queue(&self, account_unique_id: i32) -> BattleMatchResponse {
        println!("BattleRoomServiceImpl: enqueue_player_id_to_ready_queue()");

        let battle_room_ready_queue_repository = self.battle_room_ready_queue_repository.lock().await;
        battle_room_ready_queue_repository.enqueue_player_id_for_ready(account_unique_id).await;

        BattleMatchResponse::new(false)
    }
}