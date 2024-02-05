use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_round::entity::game_round::GameRound;
use crate::game_round::repository::game_round_repository::GameRoundRepository;

pub struct GameRoundRepositoryImpl {
    game_round_map: IndexMap<i32, GameRound>,
}

impl GameRoundRepositoryImpl {
    pub fn new() -> Self {
        GameRoundRepositoryImpl {
            game_round_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_round_map(&mut self) -> &mut IndexMap<i32, GameRound> {
        &mut self.game_round_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameRoundRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameRoundRepositoryImpl >> =
                Arc::new(
                    AsyncMutex::new(
                        GameRoundRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameRoundRepository for GameRoundRepositoryImpl {
    fn create_game_round_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameRoundRepositoryImpl: create_game_round_object()");

        let new_game_round = GameRound::new();
        self.game_round_map.insert(account_unique_id, new_game_round);

        true
    }

    fn next_game_round_object(&mut self, account_unique_id: i32) -> i32 {
        println!("GameRoundRepositoryImpl: next_game_round_object()");

        if let Some(index) = self.game_round_map.get_index_of(&account_unique_id) {
            if let Some((_key, game_round)) = self.game_round_map.get_index_mut(index) {
                game_round.next_round();
                return game_round.get_round();
            }
        }

        -1
    }
}

#[cfg(test)]
mod cfg_test {
    use std::time::Duration;
    use super::*;
    use tokio::test;
    use tokio::time::sleep;

    #[test]
    async fn async_test_create_game_round_object() {
        let mut repository = GameRoundRepositoryImpl::new();
        let account_unique_id = 456;

        let result = repository.create_game_round_object(account_unique_id);

        assert!(result);
        let game_round_option = repository.game_round_map.get(&account_unique_id);
        assert!(game_round_option.is_some());

        if let Some(game_round) = game_round_option {
            println!("GameRound inserted successfully!");
            println!("Account Unique ID: {}", account_unique_id);
            println!("Turn Value: {}", game_round.get_round());
        } else {
            println!("Failed to insert GameTurn!");
        }
    }

    #[test]
    async fn test_create_game_round_object() {
        let mut repository = GameRoundRepositoryImpl::new();
        let account_unique_id = 123;

        let result = repository.create_game_round_object(account_unique_id);

        assert!(result);

        let game_round_option = repository.game_round_map.get(&account_unique_id);
        assert!(game_round_option.is_some());

        if let Some(game_round) = game_round_option {
            println!("GameRound inserted successfully!");
            println!("Account Unique ID: {}", account_unique_id);
            println!("Turn Value: {}", game_round.get_round());
        } else {
            println!("Failed to insert GameTurn!");
        }
    }

    #[tokio::test]
    async fn test_async_task_in_module() {

        async fn async_task1() {
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

                println!("GameTurn Map: {:?}", game_round_map);

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

                println!("GameTurn Map: {:?}", game_round_map);

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

    #[test]
    async fn test_next_game_round_object() {
        let mut repository = GameRoundRepositoryImpl::new();
        let account_unique_id = 123;

        repository.create_game_round_object(account_unique_id);

        let result = repository.next_game_round_object(account_unique_id);

        assert_eq!(result, 2);

        let result = repository.next_game_round_object(account_unique_id);

        assert_eq!(result, 3);
    }

    #[test]
    async fn test_next_game_round_object_not_found() {
        let mut repository = GameRoundRepositoryImpl::new();
        let account_unique_id = 456;

        let result = repository.next_game_round_object(account_unique_id);

        assert_eq!(result, -1);
    }
}