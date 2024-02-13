use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use tokio::time::sleep;
use std::time::Duration;

use crate::first_turn_decision_wait_queue::service::first_turn_decision_wait_queue_service::FirstTurnDecisionWaitQueueService;
use crate::first_turn_decision_wait_queue::service::request::first_turn_decision_wait_queue_request::FirstTurnDecisionWaitQueueRequest;
use crate::first_turn_decision_wait_queue::service::response::first_turn_decision_wait_queue_response::FirstTurnDecisionWaitQueueResponse;
use crate::first_turn_decision_wait_queue::repository::first_turn_decision_wait_queue_repository::FirstTurnDecisionWaitQueueRepository;
use crate::first_turn_decision_wait_queue::repository::first_turn_decision_wait_queue_repository_impl::FirstTurnDecisionWaitQueueRepositoryImpl;
use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct FirstTurnDecisionWaitQueueServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    first_turn_decision_wait_queue_repository: Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
}

impl FirstTurnDecisionWaitQueueServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               first_turn_decision_wait_queue_repository: Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,

    ) -> Self {

        FirstTurnDecisionWaitQueueServiceImpl {
            redis_in_memory_repository,
            first_turn_decision_wait_queue_repository,
            match_waiting_timer_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        FirstTurnDecisionWaitQueueServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance(),
                            FirstTurnDecisionWaitQueueRepositoryImpl::get_instance(),
                            MatchWaitingTimerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn parse_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32")
    }
}

#[async_trait]
impl FirstTurnDecisionWaitQueueService for FirstTurnDecisionWaitQueueServiceImpl {


    async fn enqueue_player_tuple_to_wait_queue(&self, first_turn_decision_wait_queue_request: FirstTurnDecisionWaitQueueRequest) -> FirstTurnDecisionWaitQueueResponse {
        println!("FirstTurnDecisionWaitQueueServiceImpl: enqueue_player_session_id_and_choice_to_wait_queue()");

        let account_unique_id = self.parse_account_unique_id(first_turn_decision_wait_queue_request.get_session_id()).await;
        let choice=first_turn_decision_wait_queue_request.get_choice().to_string();
        let mut player_tuple: (i32, String)=(account_unique_id,choice);



        let first_turn_decision_wait_queue_repository = self.first_turn_decision_wait_queue_repository.lock().await;

        let mut match_waiting_timer_repository = self.match_waiting_timer_repository.lock().await;
        match_waiting_timer_repository.set_match_waiting_timer(account_unique_id).await;


        let response = first_turn_decision_wait_queue_repository.enqueue_player_tuple_for_wait(player_tuple).await;

        if response.is_ok() {
            return FirstTurnDecisionWaitQueueResponse::new(true)
        }

        return FirstTurnDecisionWaitQueueResponse::new(false)

    }
}