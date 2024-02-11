use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;



use crate::first_turn_decision_wait_queue::service::first_turn_decision_wait_queue_service::FirstTurnDecisionWaitQueueService;
use crate::first_turn_decision_wait_queue::service::request::first_turn_decision_wait_queue_request::FirstTurnDecisionWaitQueueRequest;
use crate::first_turn_decision_wait_queue::service::response::first_turn_decision_wait_queue_response::FirstTurnDecisionWaitQueueResponse;
use crate::first_turn_decision_wait_queue::repository::first_turn_decision_wait_queue_repository::FirstTurnDecisionWaitQueueRepository;
use crate::first_turn_decision_wait_queue::repository::first_turn_decision_wait_queue_repository_impl::FirstTurnDecisionWaitQueueRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct FirstTurnDecisionWaitQueueServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    first_turn_decision_wait_queue_repository: Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>,
}

impl FirstTurnDecisionWaitQueueServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               first_turn_decision_wait_queue_repository: Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>,

    ) -> Self {

        FirstTurnDecisionWaitQueueServiceImpl {
            redis_in_memory_repository,
            first_turn_decision_wait_queue_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        FirstTurnDecisionWaitQueueServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance(),
                            FirstTurnDecisionWaitQueueRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn parse_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        println!("###테스트설마 이거 비었냐?----> {:?}",account_unique_id_option_string);
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
        let id_length=first_turn_decision_wait_queue_repository.get_wait_queue_player_id_length().await;
        let choice_length=first_turn_decision_wait_queue_repository.get_wait_queue_player_choice_length().await;
        first_turn_decision_wait_queue_repository.enqueue_player_id_for_wait(player_unique_id).await;
        first_turn_decision_wait_queue_repository.enqueue_player_choice_for_wait(player_choice.to_string()).await;
        if id_length==2 && choice_length==2
        {

             players_id=first_turn_decision_wait_queue_repository.dequeue_two_players_id_from_wait_queue(2).await;
             players_choice=first_turn_decision_wait_queue_repository.dequeue_two_players_choice_from_wait_queue(2).await;
        }
        let player_id1=players_id[0];
        let player_id2=players_id[1];
        let player_choice1=players_choice[0].clone();
        let player_choice2=players_choice[1].clone();
        drop(first_turn_decision_wait_queue_repository);


        return FirstTurnDecisionWaitQueueResponse::new(player_id1,player_choice1,player_id2,player_choice2)
    }
}