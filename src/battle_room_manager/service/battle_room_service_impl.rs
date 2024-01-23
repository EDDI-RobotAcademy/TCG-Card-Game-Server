use std::sync::Arc;
use async_trait::async_trait;
use diesel::result::Error;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room_manager::repository::battle_room_ready_queue_repository_impl::BattleRoomReadyQueueRepositoryImpl;
use crate::battle_room_manager::repository::battle_room_wait_queue_repository::BattleRoomWaitQueueRepository;
use crate::battle_room_manager::repository::battle_room_wait_queue_repository_impl::BattleRoomWaitQueueRepositoryImpl;
use crate::battle_room_manager::service::battle_room_service::BattleRoomService;

pub struct BattleRoomServiceImpl {
    battle_room_ready_queue_repository: Arc<AsyncMutex<BattleRoomReadyQueueRepositoryImpl>>,
    battle_room_wait_queue_repository: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>>,
}

impl BattleRoomServiceImpl {
    pub fn new(battle_room_ready_queue_repository: Arc<AsyncMutex<BattleRoomReadyQueueRepositoryImpl>>,
               battle_room_wait_queue_repository: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>>
            ) -> Self {

        BattleRoomServiceImpl {
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
                            BattleRoomReadyQueueRepositoryImpl::get_instance(),
                            BattleRoomWaitQueueRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleRoomService for BattleRoomServiceImpl {

    async fn enqueue_player_id_to_wait_queue(&self, account_unique_id: i32) -> Result<bool, Error> {
        println!("BattleRoomServiceImpl: enqueue_player_id_to_wait_queue()");

        let battle_room_wait_queue_repository = self.battle_room_wait_queue_repository.lock().await;
        battle_room_wait_queue_repository.enqueue_player_id_for_wait(account_unique_id).await
    }
}