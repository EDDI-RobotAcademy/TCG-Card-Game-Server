use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;
use crate::game_turn::service::game_turn_service::GameTurnService;
use crate::game_turn::service::request::decide_first_turn_request::DecideFirstTurnRequest;
use crate::game_turn::service::response::decide_first_turn_response::DecideFirstTurnResponse;
use crate::game_turn::service::request::decide_first_turn_request::Gesture;


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
    async fn decide_first_turn_object(&mut self, decide_first_turn_request1: DecideFirstTurnRequest,
                                                 decide_first_turn_request2: DecideFirstTurnRequest) -> DecideFirstTurnResponse {
        println!("GameTurnServiceImpl: decide_first_turn_object()");
        let session_id1 = decide_first_turn_request1.get_session_id();
        let choice1 = decide_first_turn_request1.get_choice();
        let session_id2 = decide_first_turn_request2.get_session_id();
        let choice2 = decide_first_turn_request2.get_choice();

        let account_unique_id1 = self.get_account_unique_id(session_id1).await;
        let account_unique_id2 = self.get_account_unique_id(session_id2).await;

        let mut game_turn_repository_guard = self.game_turn_repository.lock().await;
        let result = game_turn_repository_guard.decide_first_turn(account_unique_id1,choice1,account_unique_id2,choice2);


        DecideFirstTurnResponse::new(result.first_player, result.result_is_draw)
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use tokio::test;
    use crate::account_deck::service::account_deck_service_impl::AccountDeckServiceImpl;
    use crate::game_turn::service::request::decide_first_turn_request::Gesture;
    use crate::game_turn::service::request::decide_first_turn_request::DecideFirstTurnRequest;
    use crate::game_turn::service::response::decide_first_turn_response::DecideFirstTurnResponse;


    #[test]
    async fn test_decide_first_turn_object() {
        let game_turn_service_mutex = GameTurnServiceImpl::get_instance();
        let mut game_turn_service_mutex_guard = game_turn_service_mutex.lock().await;
        let session_id1="test1";
        let session_id2="test2";
        let choice1=Gesture::Rock;
        let choice2=Gesture::Scissors;
        let request1 = DecideFirstTurnRequest::new(session_id1.to_string(), choice1);
        let request2 = DecideFirstTurnRequest::new(session_id2.to_string(), choice2);
        let result = game_turn_service_mutex_guard.decide_first_turn_object(request1,request2).await;
        println!("{:?}",result);
    }


}
