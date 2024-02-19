use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_round::repository::game_round_repository::GameRoundRepository;

use crate::game_round::service::game_round_service::GameRoundService;
use crate::game_round::service::request::game_turn_request::GameRoundRequest;
use crate::game_round::service::response::game_turn_response::GameRoundResponse;

use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;
use crate::game_round::service::request::get_game_round_of_account_unique_id_request::GetGameRoundOfAccountUniqueIdRequest;
use crate::game_round::service::request::next_game_turn_request::NextGameRoundRequest;
use crate::game_round::service::response::get_game_round_of_account_unique_id_response::GetGameRoundOfAccountUniqueIdResponse;
use crate::game_round::service::response::next_game_turn_response::NextGameRoundResponse;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameRoundServiceImpl {
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl GameRoundServiceImpl {
    pub fn new(game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {
        GameRoundServiceImpl {
            game_round_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameRoundServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameRoundServiceImpl >> =
                Arc::new(
                    AsyncMutex::new(
                        GameRoundServiceImpl::new(
                            GameRoundRepositoryImpl::get_instance(),
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
impl GameRoundService for GameRoundServiceImpl {
    async fn create_game_round_object(&mut self, game_round_request: GameRoundRequest) -> GameRoundResponse {
        println!("GameRoundServiceImpl: create_game_round_object()");

        let session_id = game_round_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let mut game_round_repository_guard = self.game_round_repository.lock().await;

        let result = game_round_repository_guard.create_game_round_object(account_unique_id);

        GameRoundResponse::new(result)
    }

    async fn next_game_round_object(&mut self, game_round_request: NextGameRoundRequest) -> NextGameRoundResponse {
        println!("GameRoundServiceImpl: next_game_round_object()");

        let session_id = game_round_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let mut game_round_repository_guard = self.game_round_repository.lock().await;

        let result = game_round_repository_guard.next_game_round_object(account_unique_id);

        NextGameRoundResponse::new(result)
    }

    async fn get_game_round_of_account_unique_id(&mut self, get_game_round_of_account_unique_id_request: GetGameRoundOfAccountUniqueIdRequest) -> GetGameRoundOfAccountUniqueIdResponse {
        println!("GameFieldUnitServiceImpl: get_game_field_unit_card_of_account_unique_id()");

        let account_unique_id = get_game_round_of_account_unique_id_request.get_account_unique_id();

        let mut game_round_repository_guard = self.game_round_repository.lock().await;
        let mut current_game_round = game_round_repository_guard.get_game_round_map();
        let current_game_round_map = current_game_round.get_mut(&account_unique_id).unwrap();
        let current_game_round_of_account_unique_id =current_game_round_map.get_round().clone();
        drop(game_round_repository_guard);

        return GetGameRoundOfAccountUniqueIdResponse::new(current_game_round_of_account_unique_id)
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
            let service = GameRoundServiceImpl::get_instance();

            let mut repository = GameRoundRepositoryImpl::get_instance();
            let mut repository_guard = repository.lock().await;
            let account_unique_id = 1;

            let result = repository_guard.create_game_round_object(account_unique_id);

            drop(repository_guard);

            for _ in 0..5 {
                println!("Async task1 is running!");

                let mut repository_guard = repository.lock().await;
                let account_unique_id = 1;

                let game_round_map = repository_guard.get_game_round_map();

                println!("GameRound Map: {:?}", game_round_map);

                if let Some(index) = game_round_map.get_index_of(&account_unique_id) {
                    if let Some((_key, game_round)) = game_round_map.get_index_mut(index) {
                        game_round.next_round();
                    }
                }

                drop(repository_guard);

                sleep(Duration::from_millis(500)).await;
                println!("Async task completed!");
            }
        }

        async fn async_task2() {
            let service = GameRoundServiceImpl::get_instance();

            let mut repository = GameRoundRepositoryImpl::get_instance();
            let mut repository_guard = repository.lock().await;
            let account_unique_id = 2;

            let result = repository_guard.create_game_round_object(account_unique_id);

            drop(repository_guard);

            for _ in 0..5 {
                println!("Async task2 is running!");

                let mut repository_guard = repository.lock().await;
                let account_unique_id = 2;

                let game_round_map = repository_guard.get_game_round_map();

                println!("GameRound Map: {:?}", game_round_map);

                if let Some(index) = game_round_map.get_index_of(&account_unique_id) {
                    if let Some((_key, game_round)) = game_round_map.get_index_mut(index) {
                        game_round.next_round();
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