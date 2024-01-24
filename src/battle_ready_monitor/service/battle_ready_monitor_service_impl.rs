use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use tokio::time::sleep;
use crate::battle_ready_monitor::entity::battle_ready_status::BattleReadyStatus;
use crate::battle_ready_monitor::repository::battle_ready_monitor_repository::BattleReadyMonitorRepository;
use crate::battle_ready_monitor::repository::battle_ready_monitor_repository_impl::BattleReadyMonitorRepositoryImpl;
use crate::battle_ready_monitor::service::battle_ready_monitor_service::BattleReadyMonitorService;
use crate::battle_ready_monitor::service::request::battle_ready_request::BattleReadyRequest;
use crate::battle_ready_monitor::service::response::battle_ready_response::BattleReadyResponse;
use crate::battle_room::repository::battle_room_wait_queue_repository::BattleRoomWaitQueueRepository;
use crate::battle_room::repository::battle_room_wait_queue_repository_impl::BattleRoomWaitQueueRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct BattleReadyMonitorServiceImpl {
    battle_room_wait_queue_repository: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>>,
    battle_ready_monitor_repository: Arc<AsyncMutex<BattleReadyMonitorRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl BattleReadyMonitorServiceImpl {
    pub fn new(battle_room_wait_queue_repository: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>>,
               battle_ready_monitor_repository: Arc<AsyncMutex<BattleReadyMonitorRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {

        BattleReadyMonitorServiceImpl {
            battle_room_wait_queue_repository,
            battle_ready_monitor_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleReadyMonitorServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleReadyMonitorServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleReadyMonitorServiceImpl::new(
                            BattleRoomWaitQueueRepositoryImpl::get_instance(),
                            BattleReadyMonitorRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleReadyMonitorService for BattleReadyMonitorServiceImpl {
    // TODO: 성능이 중요해지는 시점이 되면 그 때 가서 고려하자
    // async fn start_monitor(&mut self) {
    //     println!("BattleReadyMonitorServiceImpl: start_monitor()");
    //
    //     loop {
    //         println!("BattleReadyMonitorServiceImpl: start_monitor() loop!");
    //         let battle_room_wait_queue_repository_mutex = self.battle_room_wait_queue_repository.lock().await;
    //         let matched_two_players = battle_room_wait_queue_repository_mutex.dequeue_two_players_from_wait_queue(2).await;
    //
    //         if (!matched_two_players.is_empty()) {
    //             println!("BattleReadyMonitorServiceImpl: 플레이어간 배틀 성사!!!");
    //             let mut battle_ready_monitor_repository_mutex = self.battle_ready_monitor_repository.lock().await;
    //             battle_ready_monitor_repository_mutex.save_battle_account_hash(matched_two_players, BattleReadyStatus::SUCCESS).await;
    //         }
    //
    //         sleep(Duration::from_millis(300)).await;
    //     }
    // }

    // TODO: 코드 흐름이 요상해지기 때문에 실제로는 태스크 분리가 필요함
    async fn check_ready_for_battle(&self, battle_ready_request: BattleReadyRequest) -> BattleReadyResponse {
        println!("BattleReadyMonitorServiceImpl: check_ready_for_battle()");

        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(battle_ready_request.get_session_id()).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");

        let battle_room_wait_queue_repository_mutex = self.battle_room_wait_queue_repository.lock().await;
        let matched_two_players = battle_room_wait_queue_repository_mutex.dequeue_two_players_from_wait_queue(2).await;

        if (!matched_two_players.is_empty()) {
            println!("BattleReadyMonitorServiceImpl: 플레이어간 배틀 성사!!!");
            let mut battle_ready_monitor_repository_mutex = self.battle_ready_monitor_repository.lock().await;
            battle_ready_monitor_repository_mutex.save_battle_account_hash(matched_two_players, BattleReadyStatus::SUCCESS).await;
        }

        let mut battle_ready_monitor_repository_mutex = self.battle_ready_monitor_repository.lock().await;
        let response = battle_ready_monitor_repository_mutex.get_account_status(account_unique_id).await;

        return match response {
            BattleReadyStatus::SUCCESS => {
                BattleReadyResponse::new(BattleReadyStatus::SUCCESS)
            },
            BattleReadyStatus::WAIT => {
                BattleReadyResponse::new(BattleReadyStatus::WAIT)
            },
            BattleReadyStatus::FAIL => {
                BattleReadyResponse::new(BattleReadyStatus::FAIL)
            },
        }
    }
}
