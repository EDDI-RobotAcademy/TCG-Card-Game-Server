use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository::BattleReadyAccountHashRepository;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository_impl::BattleReadyAccountHashRepositoryImpl;
use crate::battle_ready_account_hash::service::battle_ready_account_hash_service::BattleReadyAccountHashService;
use crate::battle_ready_account_hash::service::request::battle_ready_account_hash_request::BattleReadyAccountHashRequest;
use crate::battle_ready_account_hash::service::response::battle_ready_account_hash_response::BattleReadyAccountHashResponse;
use crate::battle_wait_queue::repository::battle_wait_queue_repository_impl::BattleWaitQueueRepositoryImpl;

use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct BattleReadyAccountHashServiceImpl {
    battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
    battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl BattleReadyAccountHashServiceImpl {
    pub fn new(battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
               battle_ready_account_hash_repository: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {

        BattleReadyAccountHashServiceImpl {
            battle_wait_queue_repository,
            battle_ready_account_hash_repository,
            match_waiting_timer_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleReadyAccountHashServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleReadyAccountHashServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleReadyAccountHashServiceImpl::new(
                            BattleWaitQueueRepositoryImpl::get_instance(),
                            BattleReadyAccountHashRepositoryImpl::get_instance(),
                            MatchWaitingTimerRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
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

    async fn is_match_waiting_timer_expired(&self, account_unique_id: i32) -> bool {
        let mut match_waiting_timer_repository_mutex = self.match_waiting_timer_repository.lock().await;
        match_waiting_timer_repository_mutex.check_match_waiting_timer_expired(account_unique_id, Duration::from_secs(60)).await
    }
}

#[async_trait]
impl BattleReadyAccountHashService for BattleReadyAccountHashServiceImpl {

    async fn check_ready_for_battle(&self, battle_ready_account_hash_request: BattleReadyAccountHashRequest) -> BattleReadyAccountHashResponse {
        println!("BattleReadyMonitorServiceImpl: check_ready_for_battle()");

        let session_id = battle_ready_account_hash_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let is_expired = self.is_match_waiting_timer_expired(account_unique_id).await;
        if is_expired {
            let mut battle_ready_account_hash_repository_mutex = self.battle_ready_account_hash_repository.lock().await;
            battle_ready_account_hash_repository_mutex.save_battle_ready_account_hash(account_unique_id, BattleReadyAccountHashStatus::FAIL).await;
        }

        // 현재 사용자 배틀 매칭 상태값을 획득
        let mut battle_ready_account_hash_repository_mutex = self.battle_ready_account_hash_repository.lock().await;
        let response = battle_ready_account_hash_repository_mutex.get_battle_ready_account_hash_status(account_unique_id).await;

        // 확보한 상태값에 따라 SUCCESS, WAIT, FAIL 정보 응답
        return match response {
            BattleReadyAccountHashStatus::SUCCESS => {
                BattleReadyAccountHashResponse::new(BattleReadyAccountHashStatus::SUCCESS)
            },
            BattleReadyAccountHashStatus::WAIT => {
                BattleReadyAccountHashResponse::new(BattleReadyAccountHashStatus::WAIT)
            },
            BattleReadyAccountHashStatus::FAIL => {
                BattleReadyAccountHashResponse::new(BattleReadyAccountHashStatus::FAIL)
            },
        }
    }
}
