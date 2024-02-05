use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;
use crate::game_turn::service::game_turn_service::GameTurnService;
use crate::game_turn::service::request::decide_first_turn_request::DecideFirstTurnRequest;
use crate::game_turn::service::response::decide_first_turn_response::DecideFirstTurnResponse;


use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameTurnServiceImpl {
    game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,

}

impl GameTurnServiceImpl {
    pub fn new(game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {
        GameTurnServiceImpl {
            game_turn_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameTurnServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameTurnServiceImpl >> =
                Arc::new(
                    AsyncMutex::new(
                        GameTurnServiceImpl::new(
                            GameTurnRepositoryImpl::get_instance(),
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
}

#[async_trait]
impl GameTurnService for GameTurnServiceImpl {
    async fn decide_first_turn_object(&mut self, decide_first_turn_request: DecideFirstTurnRequest) -> DecideFirstTurnResponse {
        println!("GameTurnServiceImpl: decide_first_turn_object()");
        let session_id = decide_first_turn_request.get_session_id();
        let gesture = decide_first_turn_request.get_gesture();
        let account_unique_id = self.get_account_unique_id(session_id).await;
        let mut game_turn_repository_guard = self.game_turn_repository.lock().await;
        let result = game_turn_repository_guard.decide_first_turn(account_unique_id, gesture.to_string());
        DecideFirstTurnResponse::new(result.first_player, result.result_is_draw)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;



    #[test]
    async fn test_decide_first_turn() {
        let game_turn_service_mutex = GameTurnServiceImpl::get_instance();
        let mut game_turn_service_mutex_guard = game_turn_service_mutex.lock().await;

        let session_id = "session_id";
        let gesture="Rock";


        let decide_first_turn_request = DecideFirstTurnRequest::new(session_id.to_string(), gesture.to_string());

        let result = game_turn_service_mutex_guard.decide_first_turn_object(decide_first_turn_request).await;

        println!("{:?}",result);
    }


}
