use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::first_turn_decision_wait_queue::repository::first_turn_decision_wait_queue_repository_impl::FirstTurnDecisionWaitQueueRepositoryImpl;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;
use crate::game_turn::service::game_turn_service::GameTurnService;
use crate::game_turn::service::request::first_turn_decision_request::FirstTurnDecisionRequest;
use crate::game_turn::service::response::first_turn_decision_response::FirstTurnDecisionResponse;



use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameTurnServiceImpl {
    game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    first_turn_decision_wait_queue_repository:Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>

}

impl GameTurnServiceImpl {
    pub fn new(game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               first_turn_decision_wait_queue_repository:Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>>) -> Self {
        GameTurnServiceImpl {
            game_turn_repository,
            redis_in_memory_repository,
            first_turn_decision_wait_queue_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameTurnServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameTurnServiceImpl >> =
                Arc::new(
                    AsyncMutex::new(
                        GameTurnServiceImpl::new(
                            GameTurnRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
                        FirstTurnDecisionWaitQueueRepositoryImpl::get_instance())));
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
}

#[async_trait]
impl GameTurnService for GameTurnServiceImpl {
    async fn first_turn_decision_object(&mut self, decide_first_turn_request: FirstTurnDecisionRequest)-> FirstTurnDecisionResponse {
        println!("GameTurnServiceImpl: first_turn_decision_object()");
        let first_turn_decision_wait_queue_repository_guard = self.first_turn_decision_wait_queue_repository.lock().await;
        let wait_queue_clone_mutex = first_turn_decision_wait_queue_repository_guard.get_first_turn_decision_wait_queue();
        let wait_queue_clone_guard = wait_queue_clone_mutex.lock().await;
        let mut players_data = wait_queue_clone_guard.dequeue_player_tuple().await;
        let player1=players_data.unwrap();
        let mut players_data = wait_queue_clone_guard.dequeue_player_tuple().await;
        let player2=players_data.unwrap();
        let session_id = decide_first_turn_request.get_session_id();
        let account_id=self.get_account_unique_id(session_id).await;



        let mut game_turn_repository_guard = self.game_turn_repository.lock().await;
        let result = game_turn_repository_guard.decide_first_turn(account_id,player1.0,player1.1,player2.0,player2.1);
        drop(game_turn_repository_guard);

        FirstTurnDecisionResponse::new(result.0,result.1,result.2)
    }
}
