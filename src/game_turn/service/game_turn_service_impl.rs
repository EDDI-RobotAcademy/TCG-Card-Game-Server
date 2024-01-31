use std::sync::Arc;
use async_trait::async_trait;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_turn::entity::game_turn::GameTurn;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;

use crate::game_turn::service::game_turn_service::GameTurnService;
use crate::game_turn::service::request::game_turn_request::GameTurnRequest;
use crate::game_turn::service::response::game_turn_response::GameTurnResponse;

use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameTurnServiceImpl {
    game_turn_repository: Arc<AsyncMutex<GameTurnRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
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
            static ref INSTANCE: Arc<AsyncMutex<GameTurnServiceImpl>> =
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
    async fn create_game_turn_object(&mut self, game_turn_request: GameTurnRequest) -> GameTurnResponse {
        println!("GameTurnServiceImpl: create_game_turn_object");

        let session_id = game_turn_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let mut game_turn_repository_guard = self.game_turn_repository.lock().await;

        let result = game_turn_repository_guard.create_game_turn_object(account_unique_id);

        GameTurnResponse::new(result)
    }
}

#[cfg(test)]
mod cfg_test {
    use std::time::Duration;
    use super::*;
    use tokio::test;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_async_task_in_module() {

        async fn async_task1() {
            let service = GameTurnServiceImpl::get_instance();

            let mut repository = GameTurnRepositoryImpl::get_instance();
            let mut repository_guard = repository.lock().await;
            let account_unique_id = 1;

            let result = repository_guard.create_game_turn_object(account_unique_id);

            drop(repository_guard);

            for _ in 0..5 {
                println!("Async task1 is running!");

                let mut repository_guard = repository.lock().await;
                let account_unique_id = 1;

                let game_turn_map = repository_guard.get_game_turn_map();

                println!("GameTurn Map: {:?}", game_turn_map);

                if let Some(index) = game_turn_map.get_index_of(&account_unique_id) {
                    if let Some((_key, game_turn)) = game_turn_map.get_index_mut(index) {
                        game_turn.next_turn();
                    }
                }

                drop(repository_guard);

                sleep(Duration::from_millis(500)).await;
                println!("Async task completed!");
            }
        }

        async fn async_task2() {
            let service = GameTurnServiceImpl::get_instance();

            let mut repository = GameTurnRepositoryImpl::get_instance();
            let mut repository_guard = repository.lock().await;
            let account_unique_id = 2;

            let result = repository_guard.create_game_turn_object(account_unique_id);

            drop(repository_guard);

            for _ in 0..5 {
                println!("Async task2 is running!");

                let mut repository_guard = repository.lock().await;
                let account_unique_id = 2;

                let game_turn_map = repository_guard.get_game_turn_map();

                println!("GameTurn Map: {:?}", game_turn_map);

                if let Some(index) = game_turn_map.get_index_of(&account_unique_id) {
                    if let Some((_key, game_turn)) = game_turn_map.get_index_mut(index) {
                        game_turn.next_turn();
                    }
                }

                drop(repository_guard);

                sleep(Duration::from_millis(500)).await;
                println!("Async task completed!");
            }
        }

        let task_handle1 = tokio::spawn(async_task1());
        let task_handle2 = tokio::spawn(async_task2());

        println!("Test in module continues its work...");

        task_handle1.await.unwrap();
        task_handle2.await.unwrap();
    }
}