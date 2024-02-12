use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::first_turn_decision_ready_account_hash::entity::first_turn_decision_ready_account_hash_status::FirstTurnDecisionReadyAccountHashStatus;
use crate::first_turn_decision_ready_account_hash::repository::first_turn_decision_ready_account_hash_repository::FirstTurnDecisionReadyAccountHashRepository;
use crate::first_turn_decision_ready_account_hash::repository::first_turn_decision_ready_account_hash_repository_impl::FirstTurnDecisionReadyAccountHashRepositoryImpl;
use crate::first_turn_decision_ready_account_hash::service::first_turn_decision_ready_account_hash_service::FirstTurnDecisionReadyAccountHashService;
use crate::first_turn_decision_ready_account_hash::service::request::first_turn_decision_ready_account_hash_request::FirstTurnDecisionReadyAccountHashRequest;
use crate::first_turn_decision_ready_account_hash::service::request::check_first_turn_decision_prepare_request::CheckFirstTurnDecisionPrepareRequest;
use crate::first_turn_decision_ready_account_hash::service::response::first_turn_decision_ready_account_hash_response::FirstTurnDecisionReadyAccountHashResponse;
use crate::first_turn_decision_ready_account_hash::service::response::check_first_turn_decision_prepare_response::CheckFirstTurnDecisionPrepareResponse;
use crate::first_turn_decision_wait_queue::repository::first_turn_decision_wait_queue_repository_impl::FirstTurnDecisionWaitQueueRepositoryImpl;
use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct FirstTurnDecisionReadyAccountHashServiceImpl {
    first_turn_decision_wait_queue_repository: Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>,
    first_turn_decision_ready_account_hash_repository: Arc<AsyncMutex<FirstTurnDecisionReadyAccountHashRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
}

impl FirstTurnDecisionReadyAccountHashServiceImpl {
    pub fn new(first_turn_decision_wait_queue_repository: Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>,
               first_turn_decision_ready_account_hash_repository: Arc<AsyncMutex<FirstTurnDecisionReadyAccountHashRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,) -> Self {

        FirstTurnDecisionReadyAccountHashServiceImpl {
            first_turn_decision_wait_queue_repository,
            first_turn_decision_ready_account_hash_repository,
            redis_in_memory_repository,
            match_waiting_timer_repository

        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<FirstTurnDecisionReadyAccountHashServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FirstTurnDecisionReadyAccountHashServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        FirstTurnDecisionReadyAccountHashServiceImpl::new(
                            FirstTurnDecisionWaitQueueRepositoryImpl::get_instance(),
                            FirstTurnDecisionReadyAccountHashRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
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
    async fn is_match_waiting_timer_expired(&self, account_unique_id: i32) -> bool {
        let mut match_waiting_timer_repository_mutex = self.match_waiting_timer_repository.lock().await;
        match_waiting_timer_repository_mutex.check_match_waiting_timer_expired(account_unique_id, Duration::from_secs(60)).await
    }


}

#[async_trait]
impl FirstTurnDecisionReadyAccountHashService for FirstTurnDecisionReadyAccountHashServiceImpl {

    async fn check_ready_for_first_turn_decision(&self, first_turn_decision_ready_account_hash_request: FirstTurnDecisionReadyAccountHashRequest) -> FirstTurnDecisionReadyAccountHashResponse {
        println!("FirstTurnDecisionReadyMonitorServiceImpl: check_ready_for_first_turn_decision()");

        let session_id = first_turn_decision_ready_account_hash_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let is_expired = self.is_match_waiting_timer_expired(account_unique_id).await;
        if is_expired {
            let mut first_turn_decision_ready_account_hash_repository_mutex = self.first_turn_decision_ready_account_hash_repository.lock().await;
            first_turn_decision_ready_account_hash_repository_mutex.save_first_turn_decision_ready_account_hash(account_unique_id, FirstTurnDecisionReadyAccountHashStatus::FAIL).await;
        }

        // 현재 사용자 배틀 매칭 상태값을 획득
        let mut first_turn_decision_ready_account_hash_repository_mutex = self.first_turn_decision_ready_account_hash_repository.lock().await;
        let response = first_turn_decision_ready_account_hash_repository_mutex.get_first_turn_decision_ready_account_hash_status(account_unique_id).await;

        // 확보한 상태값에 따라 PREPARE, WAIT, FAIL 정보 응답
        return match response {
            FirstTurnDecisionReadyAccountHashStatus::PREPARE_PROCESS => {
                FirstTurnDecisionReadyAccountHashResponse::new(FirstTurnDecisionReadyAccountHashStatus::PREPARE_PROCESS)
            },
            FirstTurnDecisionReadyAccountHashStatus::PREPARE => {
                FirstTurnDecisionReadyAccountHashResponse::new(FirstTurnDecisionReadyAccountHashStatus::PREPARE)
            },
            FirstTurnDecisionReadyAccountHashStatus::SUCCESS => {
                FirstTurnDecisionReadyAccountHashResponse::new(FirstTurnDecisionReadyAccountHashStatus::SUCCESS)
            },
            FirstTurnDecisionReadyAccountHashStatus::WAIT => {
                FirstTurnDecisionReadyAccountHashResponse::new(FirstTurnDecisionReadyAccountHashStatus::WAIT)
            },
            FirstTurnDecisionReadyAccountHashStatus::FAIL => {
                FirstTurnDecisionReadyAccountHashResponse::new(FirstTurnDecisionReadyAccountHashStatus::FAIL)
            },
        }
    }

    async fn check_prepare_for_first_turn_decision(&self, check_first_turn_decision_prepare_request: CheckFirstTurnDecisionPrepareRequest) -> CheckFirstTurnDecisionPrepareResponse {
        println!("FirstTurnDecisionReadyMonitorServiceImpl: check_prepare_for_first_turn_decision()");

        let session_id = check_first_turn_decision_prepare_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let mut first_turn_decision_ready_account_hash_repository_mutex = self.first_turn_decision_ready_account_hash_repository.lock().await;
        let response = first_turn_decision_ready_account_hash_repository_mutex.get_first_turn_decision_ready_account_hash_status(account_unique_id).await;

        // 확보한 상태값에 따라 SUCCESS, PREPARE 정보 응답
        return match response {
            FirstTurnDecisionReadyAccountHashStatus::PREPARE => {
                CheckFirstTurnDecisionPrepareResponse::new(FirstTurnDecisionReadyAccountHashStatus::PREPARE)
            },
            FirstTurnDecisionReadyAccountHashStatus::PREPARE_PROCESS => {
                CheckFirstTurnDecisionPrepareResponse::new(FirstTurnDecisionReadyAccountHashStatus::PREPARE_PROCESS)
            },
            FirstTurnDecisionReadyAccountHashStatus::SUCCESS => {
                CheckFirstTurnDecisionPrepareResponse::new(FirstTurnDecisionReadyAccountHashStatus::SUCCESS)
            },
            _ => {
                CheckFirstTurnDecisionPrepareResponse::new(FirstTurnDecisionReadyAccountHashStatus::FAIL)
            }
        }
    }
}
