use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use tokio::time::sleep;
use crate::battle_ready_monitor::service::battle_ready_monitor_service::BattleReadyMonitorService;
use crate::battle_room::repository::battle_room_wait_queue_repository::BattleRoomWaitQueueRepository;
use crate::battle_room::repository::battle_room_wait_queue_repository_impl::BattleRoomWaitQueueRepositoryImpl;
use crate::receiver::service::server_receiver_service::ServerReceiverService;
use crate::receiver::service::server_receiver_service_impl::ServerReceiverServiceImpl;

pub struct BattleReadyMonitorServiceImpl {
    battle_room_wait_queue_repository: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>>
    // battle_ready_monitor_repository: Arc<BattleReadyMonitorRepositoryImpl>,
}

impl BattleReadyMonitorServiceImpl {
    pub fn new(battle_room_wait_queue_repository:
               Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>>) -> Self {

        BattleReadyMonitorServiceImpl {
            battle_room_wait_queue_repository,
        }
    }

    pub fn get_instance() -> Arc<BattleReadyMonitorServiceImpl> {
        lazy_static! {
            static ref INSTANCE: Arc<BattleReadyMonitorServiceImpl> =
                Arc::new(
                    BattleReadyMonitorServiceImpl::new(
                        BattleRoomWaitQueueRepositoryImpl::get_instance()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleReadyMonitorService for BattleReadyMonitorServiceImpl {
    async fn start_monitor(&mut self) {
        let battle_room_wait_queue_repository_mutex = self.battle_room_wait_queue_repository.lock().await;

        loop {
            let matched_two_players = battle_room_wait_queue_repository_mutex.dequeue_two_players_from_wait_queue(2).await;

            if (!matched_two_players.is_empty()) {
                // Ready Hash 배치
            }

            sleep(Duration::from_secs(3)).await;
        }
    }
}
