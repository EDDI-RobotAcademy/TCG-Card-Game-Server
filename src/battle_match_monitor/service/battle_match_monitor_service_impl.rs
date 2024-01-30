use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::battle_match_monitor::service::battle_match_monitor_service::BattleMatchMonitorService;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository_impl::BattleReadyAccountHashRepositoryImpl;

use tokio::sync::Mutex as AsyncMutex;

use crate::battle_ready_monitor::entity::battle_ready_status::BattleReadyStatus;
use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;
use crate::battle_wait_queue::repository::battle_wait_queue_repository_impl::BattleWaitQueueRepositoryImpl;

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
        println!("BattleMatchMonitorServiceImpl: check_battle_match()");
        let battle_wait_queue_repository_guard = self.battle_wait_queue_repository.lock().await;
        let wait_queue_clone_mutex = battle_wait_queue_repository_guard.get_battle_wait_queue();
        let wait_queue_clone_guard = wait_queue_clone_mutex.lock().await;

        let items = wait_queue_clone_guard.dequeue_n_players(2).await;

        if items.len() == 2 {
            let mut battle_ready_account_hash_repository_guard = self.battle_ready_account_hash_repository.lock().await;

            battle_ready_account_hash_repository_guard.save_battle_account_hash(items[0], BattleReadyStatus::SUCCESS).await;
            battle_ready_account_hash_repository_guard.save_battle_account_hash(items[1], BattleReadyStatus::SUCCESS).await;
            drop(battle_ready_account_hash_repository_guard);

            // TODO: 배틀룸으로 매칭된 사용자 이동 (위와 마찬가지)
            let battle_room_repository_mutex = BattleRoomRepositoryImpl::get_instance();
            let battle_room_repository_guard = battle_room_repository_mutex.lock().await;

            battle_room_repository_guard.set_player_to_battle_room(items).await.expect("전투 배치 실패");
            drop(battle_room_repository_guard);
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
