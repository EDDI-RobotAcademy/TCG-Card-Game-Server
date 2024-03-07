use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::battle_match_monitor::service::battle_match_monitor_service::BattleMatchMonitorService;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository_impl::BattleReadyAccountHashRepositoryImpl;

use tokio::sync::Mutex as AsyncMutex;

use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository::BattleReadyAccountHashRepository;

use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;
use crate::battle_wait_queue::repository::battle_wait_queue_repository_impl::BattleWaitQueueRepositoryImpl;
use crate::game_battle_field_monitor::controller::game_battle_field_monitor_controller::GameBattleFieldMonitorController;
use crate::game_battle_field_monitor::controller::game_battle_field_monitor_controller_impl::GameBattleFieldMonitorControllerImpl;
use crate::mulligan_monitor::service::mulligan_monitor_service::MulliganMonitorService;
use crate::mulligan_monitor::service::mulligan_monitor_service_impl::MulliganMonitorServiceImpl;

pub struct BattleMatchMonitorServiceImpl {
    battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
    battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
    battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
}

impl BattleMatchMonitorServiceImpl {
    pub fn new(battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
               battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
               battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>) -> Self {
        BattleMatchMonitorServiceImpl {
            battle_wait_queue_repository,
            battle_ready_account_hash_repository,
            battle_room_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleMatchMonitorServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleMatchMonitorServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleMatchMonitorServiceImpl::new(
                            BattleWaitQueueRepositoryImpl::get_instance(),
                            BattleReadyAccountHashRepositoryImpl::get_instance(),
                            BattleRoomRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleMatchMonitorService for BattleMatchMonitorServiceImpl {
    async fn check_battle_match(&self) {
        loop {
            let battle_wait_queue_repository_guard = self.battle_wait_queue_repository.lock().await;
            let wait_queue_clone_mutex = battle_wait_queue_repository_guard.get_battle_wait_queue();
            let wait_queue_clone_guard = wait_queue_clone_mutex.lock().await;

            let items = wait_queue_clone_guard.dequeue_n_players(2).await;

            if items.len() == 2 {
                let mut battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;

                battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(items[0], BattleReadyAccountHashStatus::PREPARE).await;
                battle_ready_account_hash_repository_guard.save_battle_ready_account_hash(items[1], BattleReadyAccountHashStatus::PREPARE).await;
                drop(battle_ready_account_hash_repository_guard);

                let battle_room_repository_guard = self.battle_room_repository.lock().await;

                battle_room_repository_guard.set_players_to_battle_room(items).await.expect("전투 배치 실패");
                let battle_room_count = battle_room_repository_guard.get_battle_room_count().await;
                let battle_room_number = battle_room_count - 1;
                drop(battle_room_repository_guard);

                tokio::spawn(async move {
                    let game_battle_field_monitor_controller = GameBattleFieldMonitorControllerImpl::new();
                    game_battle_field_monitor_controller.battle_field_monitoring(battle_room_count).await;
                });

                tokio::spawn(async move {
                    let mulligan_monitor_service = MulliganMonitorServiceImpl::new();
                    mulligan_monitor_service.mulligan_monitoring(battle_room_number).await;
                });
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }
}
