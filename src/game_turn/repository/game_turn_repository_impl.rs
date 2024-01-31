use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_turn::entity::game_turn::GameTurn;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;

pub struct GameTurnRepositoryImpl {
    game_turn_map: IndexMap<i32, GameTurn>,
}

impl GameTurnRepositoryImpl {
    pub fn new() -> Self {
        GameTurnRepositoryImpl {
            game_turn_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_turn_map(&mut self) -> &mut IndexMap<i32, GameTurn> {
        &mut self.game_turn_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameTurnRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameTurnRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameTurnRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameTurnRepository for GameTurnRepositoryImpl {
    fn create_game_turn_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameTurnRepositoryImpl: create_game_turn_object()");

        let new_game_turn = GameTurn::new();
        self.game_turn_map.insert(account_unique_id, new_game_turn);

        true
    }

    fn next_game_turn_object(&mut self, account_unique_id: i32) -> i32 {
        println!("GameTurnRepositoryImpl: next_game_turn_object()");

        if let Some(index) = self.game_turn_map.get_index_of(&account_unique_id) {
            if let Some((_key, game_turn)) = self.game_turn_map.get_index_mut(index) {
                game_turn.next_turn();
                return game_turn.get_turn();
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
    async fn async_test_create_game_turn_object() {
        let mut repository = GameTurnRepositoryImpl::new();
        let account_unique_id = 456;

        let result = repository.create_game_turn_object(account_unique_id);

        assert!(result);
        let game_turn = repository.game_turn_map.get(&account_unique_id);
        assert!(game_turn.is_some());

        if let Some(game_turn) = game_turn {
            println!("GameTurn inserted successfully!");
            println!("Account Unique ID: {}", account_unique_id);
            println!("Turn Value: {}", game_turn.get_turn());
        } else {
            println!("Failed to insert GameTurn!");
        }
    }

    #[test]
    async fn test_create_game_turn_object() {
        // Arrange
        let mut repository = GameTurnRepositoryImpl::new();
        let account_unique_id = 123;

        let result = repository.create_game_turn_object(account_unique_id);

        assert!(result);

        let game_turn = repository.game_turn_map.get(&account_unique_id);
        assert!(game_turn.is_some());

        if let Some(game_turn) = game_turn {
            println!("GameTurn inserted successfully!");
            println!("Account Unique ID: {}", account_unique_id);
            println!("Turn Value: {}", game_turn.get_turn());
        } else {
            println!("Failed to insert GameTurn!");
        }
    }

    #[tokio::test]
    async fn test_async_task_in_module() {

        async fn async_task1() {
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

    #[test]
    async fn test_next_game_turn_object() {
        let mut repository = GameTurnRepositoryImpl::new();
        let account_unique_id = 123;

        repository.create_game_turn_object(account_unique_id);

        let result = repository.next_game_turn_object(account_unique_id);

        assert_eq!(result, 2);

        let result = repository.next_game_turn_object(account_unique_id);

        assert_eq!(result, 3);
    }

    #[test]
    async fn test_next_game_turn_object_not_found() {
        let mut repository = GameTurnRepositoryImpl::new();
        let account_unique_id = 456;

        let result = repository.next_game_turn_object(account_unique_id);

        assert_eq!(result, -1);
    }
}