use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::first_turn_decision_ready_account_hash::entity::first_turn_decision_ready_account_hash_status::FirstTurnDecisionReadyAccountHashStatus;
use crate::first_turn_decision_ready_account_hash::repository::first_turn_decision_ready_account_hash_repository::FirstTurnDecisionReadyAccountHashRepository;
use crate::first_turn_decision_ready_account_hash::repository::first_turn_decision_ready_account_hash_repository_impl::FirstTurnDecisionReadyAccountHashRepositoryImpl;
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
    first_turn_decision_ready_account_hash_repository: Arc<AsyncMutex<FirstTurnDecisionReadyAccountHashRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
}

impl FirstTurnDecisionWaitQueueServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               first_turn_decision_wait_queue_repository: Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>,
               first_turn_decision_ready_account_hash_repository: Arc<AsyncMutex<FirstTurnDecisionReadyAccountHashRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,

    ) -> Self {

        FirstTurnDecisionWaitQueueServiceImpl {
            redis_in_memory_repository,
            first_turn_decision_wait_queue_repository,
            first_turn_decision_ready_account_hash_repository,
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
                            FirstTurnDecisionReadyAccountHashRepositoryImpl::get_instance(),
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


    async fn enqueue_player_session_id_to_wait_queue(&self, first_turn_decision_wait_queue_request: FirstTurnDecisionWaitQueueRequest) -> FirstTurnDecisionWaitQueueResponse {
        println!("FirstTurnDecisionWaitQueueServiceImpl: enqueue_player_session_id_to_wait_queue()");

        let player_unique_id = self.parse_account_unique_id(first_turn_decision_wait_queue_request.get_session_id()).await;
        let player_choice = first_turn_decision_wait_queue_request.get_choice();
        let mut players_id:Vec<i32>=Vec::new();
        let mut players_choice:Vec<String>=Vec::new();
        let first_turn_decision_wait_queue_repository = self.first_turn_decision_wait_queue_repository.lock().await;

            first_turn_decision_wait_queue_repository.enqueue_player_id_for_wait(player_unique_id).await.expect("id 큐에 저장 실패");
            first_turn_decision_wait_queue_repository.enqueue_player_choice_for_wait(player_choice.to_string()).await.expect("choice 저장 실패");

        tokio::time::sleep(Duration::from_secs(60)).await;

        players_id=first_turn_decision_wait_queue_repository.dequeue_two_players_id_from_wait_queue(2).await;
        players_choice=first_turn_decision_wait_queue_repository.dequeue_two_players_choice_from_wait_queue(2).await;


        let mut players_id:Vec<i32>=Vec::new();
        let mut players_choice:Vec<String>=Vec::new();
        let player_id1=players_id[0];
        let player_id2=players_id[1];
        let player_choice1=players_choice[0].clone();
        let player_choice2=players_choice[1].clone();
        return FirstTurnDecisionWaitQueueResponse::new(player_id1,player_choice1,player_id2,player_choice2)

    }
}