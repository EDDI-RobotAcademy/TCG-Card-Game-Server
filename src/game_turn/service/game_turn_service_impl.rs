use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_turn::repository::game_turn_repository::GameTurnRepository;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;
use crate::game_turn::service::game_turn_service::GameTurnService;
use crate::game_turn::service::request::first_turn_decision_request::FirstTurnDecisionRequest;
use crate::game_turn::service::request::next_turn_request::NextTurnRequest;
use crate::game_turn::service::response::first_turn_decision_response::FirstTurnDecisionResponse;
use crate::game_turn::service::response::next_turn_response::NextTurnResponse;


use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameTurnServiceImpl {
    game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,


}

impl GameTurnServiceImpl {
    pub fn new(game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               ) -> Self {
        GameTurnServiceImpl {
            game_turn_repository,
            redis_in_memory_repository,

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
                       )));
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


    async fn next_turn(&mut self, next_turn_request: NextTurnRequest) -> NextTurnResponse {
        println!("GameTurnServiceImpl: first_turn_decision_object()");

        let mut game_turn_repository_guard = self.game_turn_repository.lock().await;
        game_turn_repository_guard.next_game_turn(next_turn_request.get_account_unique_id());

        NextTurnResponse::new(true)
    }
}
