use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_tomb::entity::game_tomb::GameTomb;
use crate::game_tomb::repository::game_tomb_repository::GameTombRepository;

pub struct GameTombRepositoryImpl {
    game_tomb_map: IndexMap<i32, GameTomb>,
}

impl GameTombRepositoryImpl {
    pub fn new() -> Self {
        GameTombRepositoryImpl {
            game_tomb_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_tomb_map(&mut self) -> &mut IndexMap<i32, GameTomb> {
        &mut self.game_tomb_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameTombRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameTombRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameTombRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameTombRepository for GameTombRepositoryImpl {
    fn create_game_tomb_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameTombRepositoryImpl: create_game_tomb_object()");

        let new_game_tomb_map = GameTomb::new();
        self.game_tomb_map.insert(account_unique_id, new_game_tomb_map);

        true
    }
    fn add_used_card_to_tomb(&mut self, account_unique_id: i32, used_card_id: i32) -> bool {
        println!("GameTombRepositoryImpl: add_used_card_to_tomb()");

        if let Some(user_game_tomb) = self.get_game_tomb_map().get_mut(&account_unique_id) {
            user_game_tomb.add_tomb_card(used_card_id)
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_create_game_tomb_object() {
        let mut game_tomb_repository = GameTombRepositoryImpl::new();
        let result = game_tomb_repository.create_game_tomb_object(1);

        assert!(result);

        println!("Test Output: {:?}", game_tomb_repository.get_game_tomb_map());

        game_tomb_repository.add_used_card_to_tomb(1, 10);

        println!("After player1 use card : {:?}", game_tomb_repository.get_game_tomb_map())
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance = GameTombRepositoryImpl::get_instance();

        let lock = instance.lock().await;

        assert!(Arc::strong_count(&instance) > 1);
        assert_eq!(lock.game_tomb_map.len(), 0);
    }
}
